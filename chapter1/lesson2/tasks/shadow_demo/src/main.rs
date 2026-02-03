fn main() {
    let input = "    42   ";
    let number: i32 = input
        .trim()          // "42"
        .parse()         // Result<i32, _>
        .expect("Не число!");
    
    println!("Число: {}", number);
    
    let spaces = "   ";
    println!("До: '{}' (длина {})", spaces, spaces.len());
    
    let spaces = spaces.len();  // Теневое преобразование в usize
    println!("После: {} (тип: число)", spaces);
    
    let text = "  hello world  ";
    let text = text.trim();           // "hello world"
    let text = text.to_uppercase();   // "HELLO WORLD"
    let text = text.replace(" ", "_"); // "HELLO_WORLD"
    
    println!("Результат: {}", text);
}