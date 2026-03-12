// Базовый импорт
mod network {
    pub mod tcp {
        pub fn connect() {}
    }
}
use network::tcp;

// Импорт с переименованием
use std::collections::HashMap as Map;

// Глобальный импорт (осторожно)
mod oreshnik {
    pub mod bomb {
        pub fn launch() {}
    }
}
use oreshnik::bomb::*;

fn main() {
    tcp::connect();
    let mut map: Map<String, i32> = Map::new();
    launch();
}