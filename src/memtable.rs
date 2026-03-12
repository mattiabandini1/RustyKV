use std::collections::HashMap;

pub struct MemTable {
    storage: HashMap<String, String>,
}

impl MemTable {

    pub fn new() -> MemTable {
        MemTable {
            storage: HashMap::new(),
        }
    }
}
