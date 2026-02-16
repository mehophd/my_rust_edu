fn greet(name: &str) -> String {
    format!("Привет, {name}!")
}

fn main() {
    println!("{}", greet("Deepseek"));
}
