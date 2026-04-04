use std::collections::HashMap;

pub struct ResourceManager {
    registry: HashMap<String, u64>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            registry: HashMap::new()
        }
    }

    pub fn register(&mut self, resource_id: &str, created_at: u64) -> Result<(), String> {
        if self.registry.contains_key(resource_id) {
            Err("Duplicate resource".to_string())
        } else {
            self.registry.insert(resource_id.to_string(), created_at);
            Ok(())
        }
    }

    pub fn check_timeouts(&self, current_time: u64) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (key, value) in &self.registry {
            if current_time - value > 100 {
                result.push(key.clone());
            }
        }

        result
    }

    pub fn get_report<'a>(&'a self, ids: &'a [String]) -> Vec<&'a String> {
        let mut result: Vec<&String> = Vec::new();
        for value in ids {
            if self.registry.contains_key(value) {
                result.push(value);
            }
        }

        result
    }
}