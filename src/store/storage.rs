use crate::store::kvstore::KvStore;
use std::collections::HashMap;
use std::fs;
use std::io;

pub trait Storage {
    fn save(&self, path: &str) -> io::Result<()>;
    fn load(path: &str) -> io::Result<Self>
    where
        Self: Sized;
}

impl Storage for KvStore {
    fn save(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.store).unwrap();
        fs::write(path, json)
    }

    fn load(path: &str) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let store: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        Ok(KvStore { store })
    }
}
