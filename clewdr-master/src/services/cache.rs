use bytes::Bytes;
use futures::{Stream, TryStreamExt, stream};
use moka::sync::Cache;
use serde_json::Value;
use snafu::ResultExt;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, LazyLock},
};
use tokio::{spawn, sync::Mutex};
use tracing::{debug, info, warn};

use crate::{
    config::CLEWDR_CONFIG,
    error::{ClewdrError, RquestSnafu},
    types::{
        claude_message::{CreateMessageParams, Message, Role},
        gemini::request::{Chat, GeminiRequestBody, SystemInstruction, Tool},
    },
};

/// Global cache instance for storing and retrieving API responses
///
/// This static provides a singleton instance of the ClewdrCache that's accessible
/// throughout the application. It's initialized lazily when first accessed.
pub static CACHE: LazyLock<ClewdrCache> = LazyLock::new(ClewdrCache::default);

/// Cache implementation for storing and retrieving API responses
///
/// ClewdrCache provides a caching mechanism for Claude API responses to improve
/// performance and reduce redundant API calls. It uses Moka as the underlying
/// cache implementation with configurable capacity and TTL settings.
pub struct ClewdrCache {
    /// The underlying Moka cache that stores the actual response data
    moka: Cache<u64, Arc<Mutex<CachedResponse>>>,
}

impl Default for ClewdrCache {
    /// Creates a new ClewdrCache instance with preconfigured settings
    ///
    /// Initializes a Moka cache with a maximum capacity of 100 entries and
    /// a time-to-live of 10 minutes. These settings balance memory usage with
    /// performance benefits from caching.
    ///
    /// # Returns
    /// * `Self` - A new ClewdrCache instance
    fn default() -> Self {
        Self {
            moka: Cache::builder()
                .max_capacity(100)
                .time_to_live(std::time::Duration::from_secs(60 * 10))
                .build(),
        }
    }
}

impl ClewdrCache {
    /// Stores an API response stream in the cache
    ///
    /// This asynchronously consumes the provided stream, converts it to a vector of bytes,
    /// and stores it in the cache using the provided key. If the cache is full for this key
    /// (based on the configured cache_response limit), the operation will be skipped.
    ///
    /// # Arguments
    /// * `stream` - The stream of bytes to cache
    /// * `key` - The hash key to store the response under
    /// * `id` - An identifier for logging purposes
    pub fn push(
        &'static self,
        stream: impl Stream<Item = Result<Bytes, wreq::Error>> + Send + 'static,
        key: u64,
        id: usize,
    ) {
        spawn(async move {
            debug!("Storing response in cache for key {}, id {}", key, id);
            let Ok(vec) = stream_to_vec(stream).await else {
                warn!(
                    "Failed to convert stream to vector for key {}, id {}",
                    key, id
                );
                return;
            };
            debug!(
                "Stream converted to vector of length: {} for key {}, id {}",
                vec.len(),
                key,
                id
            );
            if vec.is_empty() {
                return;
            }
            let value = self.moka.get_with(key, Default::default);
            debug!("Cache entry retrieved for key {}, id {}", key, id);
            {
                let mut value = value.lock().await;
                debug!("Lock acquired for cache entry for key {}, id {}", key, id);
                if value.len() >= CLEWDR_CONFIG.load().cache_response {
                    debug!("Cache is full, skipping cache for key: {}, id {}", key, id);
                    return;
                }
                debug!("Adding response to cache for key {}, id {}", key, id);
                value.push(vec);
            }
            info!("Cached response for key: {}, id {}", key, id);
        });
    }

    /// Retrieves and removes a cached response for the given key
    ///
    /// This method attempts to retrieve a cached response for the provided key.
    /// If found, it removes the entry from the cached responses and returns it as a stream.
    /// If the cache becomes empty for this key after removal, the entire cache entry is invalidated.
    ///
    /// # Arguments
    /// * `key` - The hash key to retrieve the response for
    ///
    /// # Returns
    /// * `Option<impl Stream<Item = Result<Bytes, wreq::Error>> + Send + 'static>` - The cached response as a stream, if available
    pub async fn pop(
        &self,
        key: u64,
    ) -> Option<impl Stream<Item = Result<Bytes, wreq::Error>> + Send + 'static> {
        let value = self.moka.get(&key)?;
        let (vec, now_empty) = {
            let mut value = value.lock().await;
            (value.pop(), value.is_empty())
        };
        if now_empty {
            debug!("Cache is empty for key: {}", key);
            // remove the cache entry
            self.moka.invalidate(&key);
        }
        let vec = vec?;
        Some(vec_to_stream(vec))
    }
}

/// Container for cached response data
///
/// Stores multiple response bodies as vectors of bytes, allowing for
/// storage and retrieval of stream-based API responses.
#[derive(Clone, Debug, Default)]
struct CachedResponse {
    /// Collection of cached response bodies
    bodies: Vec<Vec<Bytes>>,
}

impl CachedResponse {
    /// Returns the number of cached response bodies
    pub fn len(&self) -> usize {
        self.bodies.len()
    }

    /// Checks if there are any cached response bodies
    pub fn is_empty(&self) -> bool {
        self.bodies.is_empty()
    }

    /// Removes and returns the most recently added response body
    pub fn pop(&mut self) -> Option<Vec<Bytes>> {
        self.bodies.pop()
    }

    /// Adds a new response body to the cache
    pub fn push(&mut self, vec: Vec<Bytes>) {
        self.bodies.push(vec);
    }
}

/// Converts a stream of byte results to a vector of bytes
///
/// This utility function consumes a stream of `Result<Bytes, wreq::Error>`
/// and collects all successful results into a vector of Bytes, filtering out errors.
///
/// # Arguments
/// * `stream` - The input stream to convert
///
/// # Returns
/// * `Vec<Bytes>` - Vector containing all successful byte chunks from the stream
async fn stream_to_vec(
    stream: impl Stream<Item = Result<Bytes, wreq::Error>> + Send + 'static,
) -> Result<Vec<Bytes>, ClewdrError> {
    stream.try_collect().await.context(RquestSnafu {
        msg: "Failed to collect stream into vector",
    })
}

/// Converts a vector of bytes to a stream of successful results
///
/// This utility function takes a vector of Bytes and transforms it into
/// a stream of `Result<Bytes, wreq::Error>` where each item is wrapped in Ok.
///
/// # Arguments
/// * `bytes` - The vector of bytes to convert to a stream
///
/// # Returns
/// * `impl Stream<Item = Result<Bytes, wreq::Error>>` - Stream of successful byte results
fn vec_to_stream(bytes: Vec<Bytes>) -> impl Stream<Item = Result<Bytes, wreq::Error>> {
    stream::iter(bytes.into_iter().map(Ok))
}

/// Represents the key components of a request for caching purposes
///
/// This struct contains the essential parameters that determine the uniqueness
/// of a request for caching. It implements Hash to generate consistent hash keys
/// for the cache.
#[derive(Hash, Eq, PartialEq, Debug)]
struct ClaudeRequestKeys<'a> {
    /// Maximum number of tokens to generate
    pub max_tokens: u32,
    /// Input messages for the conversation
    pub messages: Vec<&'a Message>,
    /// Model to use
    pub model: String,
    /// System prompt
    pub system: Option<&'a Value>,
    /// Custom stop sequences
    pub stop_sequences: Option<Vec<String>>,
    /// Thinking mode configuration
    pub thinking: bool,
    /// Top-k sampling
    pub top_k: Option<u32>,
}

#[derive(Hash, Debug)]
pub struct GeminiRequestKeys<'a> {
    pub system_instruction: Option<&'a SystemInstruction>,
    pub tools: Option<&'a [Tool]>,
    pub contents: Vec<&'a Chat>,
    pub generation_config: Option<&'a Value>,
}

pub trait GetHashKey {
    /// Generates a hash value for this request key set
    ///
    /// Creates a consistent hash value for the request keys to be used
    /// as a cache key.
    ///
    /// # Returns
    /// * `u64` - The hash value for this request
    fn get_hash(&self) -> u64;
}

impl GeminiRequestKeys<'_> {
    /// Generates a hash value for this request key set
    ///
    /// Creates a consistent hash value for the request keys to be used
    /// as a cache key.
    ///
    /// # Returns
    /// * `u64` - The hash value for this request
    fn get_hash(&mut self) -> u64 {
        if CLEWDR_CONFIG.load().not_hash_system {
            self.system_instruction = None;
        }
        let end = self
            .contents
            .len()
            .saturating_sub(CLEWDR_CONFIG.load().not_hash_last_n);
        if end != 0 {
            self.contents = self.contents[..end].to_vec();
        } else {
            warn!("not_hash_last_n is too big, no messages will left, skipping");
        }
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<'a> GeminiRequestBody {
    fn to_keys(&'a self) -> GeminiRequestKeys<'a> {
        GeminiRequestKeys {
            system_instruction: self.system_instruction.as_ref(),
            tools: self.tools.as_deref(),
            contents: self.contents.iter().collect(),
            generation_config: self.generation_config.as_ref(),
        }
    }
}

impl GetHashKey for GeminiRequestBody {
    /// Generates a cache key hash from request parameters
    ///
    /// Converts the request parameters to RequestKeys and computes a hash
    /// value for use as a cache key.
    ///
    /// # Returns
    /// * `u64` - The hash value to use as a cache key
    fn get_hash(&self) -> u64 {
        let mut keys = self.to_keys();
        keys.get_hash()
    }
}

impl ClaudeRequestKeys<'_> {
    /// Generates a hash value for this request key set
    ///
    /// Creates a consistent hash value for the request keys to be used
    /// as a cache key.
    ///
    /// # Returns
    /// * `u64` - The hash value for this request
    fn get_hash(&mut self) -> u64 {
        if CLEWDR_CONFIG.load().not_hash_system {
            self.system = None;
            self.messages.retain(|m| m.role != Role::System);
        }
        let end = self
            .messages
            .len()
            .saturating_sub(CLEWDR_CONFIG.load().not_hash_last_n);
        if end != 0 {
            self.messages = self.messages[..end].to_vec();
        } else {
            warn!("not_hash_last_n is too big, no messages will left, skipping");
        }
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl GetHashKey for CreateMessageParams {
    /// Generates a cache key hash from request parameters
    ///
    /// Converts the request parameters to RequestKeys and computes a hash
    /// value for use as a cache key.
    ///
    /// # Returns
    /// * `u64` - The hash value to use as a cache key
    fn get_hash(&self) -> u64 {
        let mut keys = ClaudeRequestKeys::from(self);
        keys.get_hash()
    }
}

impl<'a> From<&'a CreateMessageParams> for ClaudeRequestKeys<'a> {
    // TODO: handle useless parameters
    fn from(params: &'a CreateMessageParams) -> Self {
        ClaudeRequestKeys {
            max_tokens: params.max_tokens,
            messages: params.messages.iter().collect(),
            model: params.model.to_owned(),
            system: params.system.as_ref(),
            stop_sequences: params.stop_sequences.to_owned(),
            thinking: params.thinking.is_some(),
            top_k: params.top_k,
        }
    }
}
