use std::collections::HashMap;

pub trait CacheEntry {
    fn key(&self) -> &str;
    fn size_bytes(&self) -> usize;
    fn on_evict(&self);  // вызывается при вытеснении из кэша
}

pub struct DataEntry {
    key: String,
    data: Vec<u8>,
    pub priority: u8
}

impl DataEntry {
    pub fn new(key: &str, data: Vec<u8>, priority: u8) -> Self {
        DataEntry {
            key: key.to_string(),
            data: data,
            priority: priority
        }
    }
}

impl CacheEntry for DataEntry {
    fn key(&self) -> &str {
        &self.key
    }
    fn size_bytes(&self) -> usize {
        self.data.len()
    }
    fn on_evict(&self) {
        println!("[EVICT] Data entry '{}' freed {} bytes", self.key, self.size_bytes());
    }
}

pub struct ConfigEntry {
    key: String,
    config: HashMap<String, String>,
    version: u32
}

impl ConfigEntry {
    pub fn new(key: &str, config: HashMap<String, String>, version: u32) -> Self {
        ConfigEntry {
            key: key.to_string(),
            config: config,
            version: version
        }
    }
}

impl CacheEntry for ConfigEntry {
    fn key(&self) -> &str {
        &self.key
    }
    fn size_bytes(&self) -> usize {
        self.config.len() * 32
    }
    fn on_evict(&self) {
        println!("[EVICT] Config entry '{}' v{} removed", self.key, self.version);
    }
}

impl Drop for DataEntry {
    fn drop(&mut self) {
        println!("[DROP] Low-priority entry '{}' cleaned", self.key);
    } 
}