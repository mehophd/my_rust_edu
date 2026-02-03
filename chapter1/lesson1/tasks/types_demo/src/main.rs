fn main() {
    let tiny: i8 = 127;        
    let small: i16 = 32767;
    let normal: i32 = 2147483647;
    let big: i64 = 9223372036854775807;
    
    println!("i8 max: {}", tiny);
    println!("i32 max: {}", normal);
    
    let unsigned: u32 = 4294967295;  
    println!("u32 max: {}", unsigned);
    
    let pi: f32 = 3.1415927;   
    let precise_pi: f64 = 3.141592653589793;  
    
    println!("\nf32 pi: {:.6}", pi);
    println!("f64 pi: {:.15}", precise_pi);
    
    let is_rust_cool = true;
    let is_hard = false;
    
    println!("\nRust крут? {}", is_rust_cool);
    println!("Rust сложен? {}", is_hard);
    
    let emoji = '🦀';
    let letter = 'A';
    
    println!("\nСимвол: {}", letter);
    println!("Эмодзи: {}", emoji);
    
    let static_str: &str = "Это строковый литерал (&str)";
    let owned_string: String = String::from("Это владеющая строка (String)");
    
    println!("\n&str: {}", static_str);
    println!("String: {}", owned_string);
    
    let inferred = 42;  
    println!("\nАвтоматический тип: {}", inferred);
}
