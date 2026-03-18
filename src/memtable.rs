use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};

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

    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(value) = self.storage.get(key) {
            return Some(value.clone());
        }

        if let Some(file) = File::open("sstable.txt").ok() {
            let reader = BufReader::new(file);

            for row_result in reader.lines() {
                if let Ok(row) =  row_result {
                    if let Some((key_saved, value_saved)) = row.split_once('=') {
                        if key_saved == key {
                            return Some(value_saved.to_string());
                        }
                    }
                }
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn flush_to_disk(&self) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("sstable.txt")
            .expect("Critical error: unable to open or create file on disk.");
        
        for (key, value) in &self.storage {
            writeln!(file, "{}={}", key, value).expect("Errors writing in file");
        }
    }
}
