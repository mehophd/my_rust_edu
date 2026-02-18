use std::collections::HashMap;

struct Cache {
    data: HashMap<String, i32>
}

impl Cache {
    fn new() -> Self {
        Cache {data: HashMap::new()}
    }

    fn get(&mut self, key: &str, compute: fn(&str) -> i32) -> i32 {
        match self.data.get(key) {
            Some(value) => *value,
            None => {
                let result = compute(key);
                self.data.insert(key.to_string(), result);
                result
            }
        }
    }
}

fn len(s: &str) -> i32 { s.len() as i32 }

fn main() {
    let mut cache = Cache::new();
    println!("{}", cache.get("hello", len)); // 5
    println!("{}", cache.get("hello", len)); // 5
}
