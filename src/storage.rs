use std::sync::Arc;
use dashmap::DashMap;
use anyhow::Result;

/// The Storage struct represents our key-value store
#[derive(Clone)]
pub struct Storage {
    data: Arc<DashMap<String, Vec<u8>>>,
}

impl Storage {
    /// Create a new Storage instance
    pub fn new() -> Self {
        Storage {
            data: Arc::new(DashMap::new()),
        }
    }

    /// Set a key-value pair in the store
    pub fn set(&self, key: String, value: Vec<u8>) -> Result<()> {
        self.data.insert(key, value);
        Ok(())
    }

    /// Get a value by key from the store
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.data.get(key).map(|v| v.clone())
    }

    /// Delete a key from the store
    pub fn delete(&self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}