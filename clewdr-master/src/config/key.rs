use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref, sync::LazyLock};
use tracing::warn;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct GeminiKey {
    pub inner: String,
}

impl Deref for GeminiKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl GeminiKey {
    pub fn validate(&self) -> bool {
        static RE: LazyLock<regex::Regex> =
            LazyLock::new(|| regex::Regex::new(r"^AIzaSy[A-Za-z0-9_-]{33}$").unwrap());
        RE.is_match(&self.inner)
    }
    pub fn ellipse(&self) -> String {
        let len = self.inner.len();
        if len > 10 {
            format!("{}...", &self.inner[..10])
        } else {
            self.inner.to_owned()
        }
    }
}

impl Display for GeminiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<S> From<S> for GeminiKey
where
    S: AsRef<str>,
{
    /// Create a new key from a string
    fn from(original: S) -> Self {
        let original = original.as_ref();
        // only keep '=' '_' '-' and alphanumeric characters
        let original = original
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '=' || *c == '_' || *c == '-')
            .collect::<String>();
        let key = Self { inner: original };
        if !key.validate() {
            warn!("Invalid key format: {}", key);
        }
        key
    }
}
impl From<GeminiKey> for String {
    /// Convert the key to a string
    fn from(key: GeminiKey) -> Self {
        key.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyStatus {
    pub key: GeminiKey,
    #[serde(default)]
    pub count_403: u32,
}

impl PartialEq for KeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for KeyStatus {}
impl std::hash::Hash for KeyStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl KeyStatus {
    pub fn validate(&self) -> bool {
        self.key.validate()
    }
}
