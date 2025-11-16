use super::crypto::{decrypt, encrypt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct KvStore {
    pub store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        let encrypted = encrypt(value.as_bytes());
        self.store.insert(key, encrypted);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).and_then(|enc| {
            decrypt(enc) // Result<Vec<u8>, &str>
                .ok() // convert to Option<Vec<u8>>
                .and_then(|bytes| String::from_utf8(bytes).ok())
        })
    }

    pub fn list(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }
}
