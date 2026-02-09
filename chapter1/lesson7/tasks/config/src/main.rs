struct Config {
    name: Option<String>,
    port: Option<u16>,
    debug_mode: bool
}

impl Config {
    fn print_summary(&self) {
        match &self.name {
            Some(name) => println!("Имя: {}", name), // используем ссылку чтобы добраться до строки и форматировать
            None => println!("Имя не указано"),
        }
    }
}
fn main() {
    let conf = Config {
        name: Some(String::from("Vasya")),
        port: Some(1488),
        debug_mode: true
    };

    conf.print_summary(); // Имя: Vasya
}
