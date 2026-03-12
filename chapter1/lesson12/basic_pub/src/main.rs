mod config {
    pub fn load() {}      // видна извне модуля
    fn validate() {}      // приватная — только внутри `config`
    
    pub struct Settings {
        pub host: String,       // публичное поле
        port: u16,              // приватное поле
    }
    
    impl Settings {
        pub fn new(host: String, port: u16) -> Settings {
            Settings { host, port }
        }
        
        // Геттер для приватного поля
        pub fn port(&self) -> u16 {
            self.port
        }
    }
}

fn main() {
    let settings = config::Settings::new("localhost".to_string(), 8080);
    println!("Порт: {}", settings.port()); // Порт: 8080
}
