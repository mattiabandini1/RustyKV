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

            let mut last_value: Option<String> = None;

            for row_result in reader.lines() {
                if let Ok(row) =  row_result {
                    if let Some((key_saved, value_saved)) = row.split_once('=') {
                        if key_saved == key {
                            last_value = Some(value_saved.to_string());
                        }
                    }
                }
            }
            
            if last_value.is_some() {
                return last_value;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memtable_set_and_get_in_memory() {
        let mut db = MemTable::new();
        
        db.set("test_key".to_string(), "test_value".to_string());

        let result = db.get("test_key");
        assert_eq!(result, Some("test_value".to_string()));

        let missing = db.get("ghost_key");
        assert_eq!(missing, None);
    }
}
