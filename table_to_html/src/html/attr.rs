/// Attribute represents a HTML `key=value` attribute pair.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
    /// Creates a new attribute.
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns a key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns a value.
    pub fn value(&self) -> &str {
        &self.value
    }
}
