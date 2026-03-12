use std::collections::HashMap;

pub struct SafeCache {
    data: HashMap<String, String>
}

impl SafeCache {
    pub fn new() -> Self {
        SafeCache { data: HashMap::new()}
    }
    pub fn set(&mut self, key:String, value:String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn is_included(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn print(&self) {
        for (key, value) in &self.data {
            println!("({}, {})", key, value);
        }
    }
}