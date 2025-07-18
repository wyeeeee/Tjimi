use colored::Colorize;
use serde::Serialize;
use std::collections::VecDeque;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
};
use tracing::{error, info};

use crate::{
    config::{CLEWDR_CONFIG, ClewdrConfig, KeyStatus},
    error::ClewdrError,
};

#[derive(Debug, Serialize, Clone)]
pub struct KeyStatusInfo {
    pub valid: Vec<KeyStatus>,
}

/// Unified event enum for key management
#[derive(Debug)]
pub enum KeyEvent {
    /// Return a Key
    Return(KeyStatus),
    /// Submit a new Key
    Submit(KeyStatus),
    /// Request to get a Key
    Request(oneshot::Sender<Result<KeyStatus, ClewdrError>>),
    /// Get all Key status information
    GetStatus(oneshot::Sender<KeyStatusInfo>),
    /// Delete a Key
    Delete(KeyStatus, oneshot::Sender<Result<(), ClewdrError>>),
}

/// Key manager that handles key distribution and status tracking
pub struct KeyManager {
    valid: VecDeque<KeyStatus>,
    event_rx: mpsc::UnboundedReceiver<KeyEvent>, // Event receiver for incoming events
}

/// Event sender interface provided for external components to interact with the key manager
#[derive(Clone)]
pub struct KeyEventSender {
    sender: mpsc::UnboundedSender<KeyEvent>,
}

impl KeyEventSender {
    /// Request a key from the key manager
    ///
    /// # Returns
    /// * `Result<KeyStatus, ClewdrError>` - Key if available, error otherwise
    pub async fn request(&self) -> Result<KeyStatus, ClewdrError> {
        let (tx, rx) = oneshot::channel();
        self.sender.send(KeyEvent::Request(tx))?;
        rx.await?
    }

    /// Return a key to the key manager
    ///
    /// # Arguments
    /// * `key` - The key to return
    ///
    /// # Returns
    /// Result indicating success or send error
    pub async fn return_key(&self, key: KeyStatus) -> Result<(), ClewdrError> {
        Ok(self.sender.send(KeyEvent::Return(key))?)
    }

    /// Submit a new key to the key manager
    ///
    /// # Arguments
    /// * `key` - The new key to add
    ///
    /// # Returns
    /// Result indicating success or send error
    pub async fn submit(&self, key: KeyStatus) -> Result<(), ClewdrError> {
        Ok(self.sender.send(KeyEvent::Submit(key))?)
    }

    /// Get status information about all keys
    ///
    /// # Returns
    /// * `Result<KeyStatusInfo, ClewdrError>` - Status information about all keys
    pub async fn get_status(&self) -> Result<KeyStatusInfo, ClewdrError> {
        let (tx, rx) = oneshot::channel();
        self.sender.send(KeyEvent::GetStatus(tx))?;
        Ok(rx.await?)
    }

    /// Delete a key from the key manager
    ///
    /// # Arguments
    /// * `key` - The key to delete
    ///
    /// # Returns
    /// * `Result<(), ClewdrError>` - Success or error
    pub async fn delete_key(&self, key: KeyStatus) -> Result<(), ClewdrError> {
        let (tx, rx) = oneshot::channel();
        self.sender.send(KeyEvent::Delete(key, tx))?;
        rx.await?
    }
}

impl KeyManager {
    /// Starts the key manager and returns an event sender
    ///
    /// Initializes key collections, creates event channels and queues,
    /// and spawns the event processing task
    ///
    /// # Returns
    /// * `KeyEventSender` - Event sender for interacting with the key manager
    pub fn start() -> KeyEventSender {
        let valid = VecDeque::from_iter(CLEWDR_CONFIG.load().gemini_keys.iter().cloned());

        // Create event channel
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let sender = KeyEventSender { sender: event_tx };

        let manager = Self { valid, event_rx };
        // Start event processor
        spawn(manager.run());

        sender
    }

    /// Logs the current state of key collections
    /// Displays count of valid keys
    fn log(&self) {
        info!("Valid Keys: {}", self.valid.len().to_string().green());
    }

    /// Saves the current state of keys to the configuration
    /// Updates the key arrays in the config and writes to disk
    fn save(&mut self) {
        CLEWDR_CONFIG.rcu(|config| {
            let mut config = ClewdrConfig::clone(config);
            config.gemini_keys = self.valid.iter().cloned().collect();
            config
        });
        tokio::spawn(async move {
            let result = CLEWDR_CONFIG.load().save().await;

            match result {
                Ok(_) => info!("Configuration saved successfully"),
                Err(e) => error!("Save task panicked: {}", e),
            }
        });
    }

    /// Dispatches a key for use
    /// Gets a key from the valid collection
    ///
    /// # Returns
    /// * `Result<KeyStatus, ClewdrError>` - A key if available, error otherwise
    fn dispatch(&mut self) -> Result<KeyStatus, ClewdrError> {
        let key = self.valid.pop_front().ok_or(ClewdrError::NoKeyAvailable)?;
        self.valid.push_back(key.to_owned());
        Ok(key)
    }

    fn collect(&mut self, key: KeyStatus) {
        // find the key and replace it
        let Some(pos) = self.valid.iter().position(|k| *k == key) else {
            error!("Key not found in valid keys");
            return;
        };
        self.valid[pos] = key;
    }

    /// Accepts a new key into the valid collection
    /// Checks for duplicates before adding
    ///
    /// # Arguments
    /// * `key` - The new key to accept
    fn accept(&mut self, key: KeyStatus) {
        if CLEWDR_CONFIG.load().gemini_keys.contains(&key) {
            info!("Key already exists");
            return;
        }
        self.valid.push_back(key);
        self.save();
        self.log();
    }

    /// Creates a report of all key statuses
    ///
    /// # Returns
    /// * `KeyStatusInfo` - Information about all key collections
    fn report(&self) -> KeyStatusInfo {
        KeyStatusInfo {
            valid: self.valid.iter().cloned().collect(),
        }
    }

    /// Deletes a key from the collection
    ///
    /// # Arguments
    /// * `key` - The key to delete
    ///
    /// # Returns
    /// * `Result<(), ClewdrError>` - Success if found and deleted, error otherwise
    fn delete(&mut self, key: KeyStatus) -> Result<(), ClewdrError> {
        let size_before = self.valid.len();
        self.valid.retain(|k| *k != key);

        if self.valid.len() < size_before {
            // Update config to reflect changes
            self.save();
            self.log();
            Ok(())
        } else {
            Err(ClewdrError::UnexpectedNone {
                msg: "Delete operation did not find the key",
            })
        }
    }

    /// Main event processing loop
    /// Processes events based on type
    ///
    /// # Arguments
    /// * `event_rx` - Event receiver for incoming events
    async fn run(mut self) {
        // Event processing main loop
        self.log();
        while let Some(event) = self.event_rx.recv().await {
            match event {
                KeyEvent::Return(key) => {
                    // Process returned key
                    self.collect(key);
                }
                KeyEvent::Submit(key) => {
                    // Process submitted new key
                    self.accept(key);
                }
                KeyEvent::Request(sender) => {
                    // Process request
                    let key = self.dispatch();
                    spawn(async move {
                        sender.send(key).unwrap_or_else(|_| {
                            error!("Failed to send key");
                        });
                    });
                }
                KeyEvent::GetStatus(sender) => {
                    let status_info = self.report();
                    spawn(async move {
                        sender.send(status_info).unwrap_or_else(|_| {
                            error!("Failed to send status info");
                        });
                    });
                }
                KeyEvent::Delete(key, sender) => {
                    let result = self.delete(key);
                    spawn(async move {
                        sender.send(result).unwrap_or_else(|_| {
                            error!("Failed to send delete result");
                        });
                    });
                }
            }
        }
    }
}
