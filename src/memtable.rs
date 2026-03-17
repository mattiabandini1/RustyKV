use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct MemTable {
    storage: HashMap<String, String>,
}

impl MemTable {

    pub fn new() -> MemTable {
        MemTable {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn flush_to_disk(&self) {

    }
}
