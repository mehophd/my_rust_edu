fn main() {
    let x = 5;          // x: i32 = 5
    let x = x + 1;      // x: i32 = 6 (новая переменная с тем же именем)
    let x = x * 2;      // x: i32 = 12
    
    println!("Значение x: {}", x);  // 12
    
    // Можно менять тип!
    let spaces = "   ";     // spaces: &str
    let spaces = spaces.len();  // spaces: usize
    
    println!("Длина: {}", spaces);  // 3
}
