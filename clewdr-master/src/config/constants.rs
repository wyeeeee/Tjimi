use arc_swap::ArcSwap;
use clap::Parser;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::LazyLock,
};
use tracing::error;
use url::Url;

use crate::{config::ClewdrConfig, utils::set_clewdr_dir};

pub const CONFIG_NAME: &str = "clewdr.toml";
pub const CLAUDE_ENDPOINT: &str = "https://api.anthropic.com";
pub const GEMINI_ENDPOINT: &str = "https://generativelanguage.googleapis.com";
pub const CC_CLIENT_ID: &str = "9d1c250a-e61b-44d9-88ed-5944d1962f5e";
pub const CC_TOKEN_URL: &str = "https://console.anthropic.com/v1/oauth/token";
pub const CC_REDIRECT_URI: &str = "https://console.anthropic.com/oauth/code/callback";

pub static ENDPOINT_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse(CLAUDE_ENDPOINT).unwrap_or_else(|_| {
        panic!("Failed to parse endpoint URL: {CLAUDE_ENDPOINT}");
    })
});
pub const LOG_DIR: &str = "log";
pub static CLEWDR_CONFIG: LazyLock<ArcSwap<ClewdrConfig>> = LazyLock::new(|| {
    let _ = *CLEWDR_DIR;
    let config = ClewdrConfig::new();
    ArcSwap::from_pointee(config)
});

pub static ARG_COOKIE_FILE: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    let args = crate::Args::parse();
    if let Some(cookie_file) = args.file {
        // canonicalize the path
        if !cookie_file.exists() {
            error!("No cookie file found at: {}", cookie_file.display());
            return None;
        }
        cookie_file.canonicalize().ok()
    } else {
        None
    }
});

pub static ARG_CONFIG_FILE: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    let args = crate::Args::parse();
    if let Some(config_file) = args.config {
        // canonicalize the path
        config_file.canonicalize().ok()
    } else {
        None
    }
});

pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Some(path) = ARG_CONFIG_FILE.as_ref() {
        path.to_owned()
    } else {
        CLEWDR_DIR.join(CONFIG_NAME)
    }
});

pub static CLEWDR_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| set_clewdr_dir().expect("Failed to get dir"));

// Default functions
/// Default number of maximum retries for API requests
///
/// # Returns
/// * `usize` - The default value of 5
pub const fn default_max_retries() -> usize {
    5
}

/// Default IP address for the server to bind to
///
/// # Returns
/// * `String` - The default localhost IP "127.0.0.1"
pub fn default_ip() -> IpAddr {
    Ipv4Addr::new(127, 0, 0, 1).into()
}

/// Default port for the server to listen on
///
/// # Returns
/// * `u16` - The default port number 8484
pub fn default_port() -> u16 {
    8484
}

/// Default setting for using real roles in conversations
///
/// # Returns
/// * `bool` - The default value of true
pub const fn default_use_real_roles() -> bool {
    true
}

/// Default length of padding text
///
/// # Returns
/// * `usize` - The default value of 4000 tokens
pub const fn default_padtxt_len() -> usize {
    4000
}

/// Default setting for checking updates on startup
///
/// # Returns
/// * `bool` - The default value of true
pub const fn default_check_update() -> bool {
    true
}
/// Default setting for skipping cool down cookies
///
/// # Returns
/// * `bool` - The default value of true
pub const fn default_skip_cool_down() -> bool {
    true
}

/// Default cookie value for testing purposes
pub const PLACEHOLDER_COOKIE: &str = "sk-ant-sid01----------------------------SET_YOUR_COOKIE_HERE----------------------------------------AAAAAAAA";
