use std::collections::HashMap;
use crate::cache_entry::DataEntry;
use crate::cache_entry::CacheEntry;
pub struct SimpleCache {
    entries: HashMap<String, DataEntry>,
    capacity_bytes: usize,
    current_usage: usize
}

impl SimpleCache {
    pub fn new(capacity_bytes: usize) -> Self {
        SimpleCache {
            entries: HashMap::new(),
            capacity_bytes,
            current_usage: 0
        }
    }

    pub fn insert(&mut self, entry: DataEntry) -> Result<(), String> {
        let entry_size = entry.size_bytes();  // через трейт CacheEntry
        
        if self.current_usage + entry_size > self.capacity_bytes {
            if self.evict_one().is_none() {
                return Err("Cache full: cannot evict to make room".to_string());
            }
        }
        
        let key = entry.key().to_string();
        self.current_usage += entry_size;
        self.entries.insert(key, entry);
        Ok(())
    }
    fn evict_one(&mut self) -> Option<String> {
        if self.entries.is_empty() {
            return None;
        }

        let mut min_priority: u8 = u8::MAX;
        let mut key_to_evict: Option<String> = None;

        for (key, entry) in &self.entries {
            if entry.priority < min_priority {
                min_priority = entry.priority;
                key_to_evict = Some(key.clone());
            }
        }

        if let Some(key) = key_to_evict {
            if let Some(entry) = self.entries.remove(&key) {
                CacheEntry::on_evict(&entry);
                self.current_usage -= entry.size_bytes();
                return Some(key);
            }
        }

        None
    }
    pub fn get<'a>(&'a self, key: &'a str) -> Option<&'a DataEntry> {
        self.entries.get(key) 
    }
}
