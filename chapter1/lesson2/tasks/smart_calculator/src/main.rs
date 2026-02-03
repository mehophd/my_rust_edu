fn calculate(a: f64, b: f64, operation: &str) -> f64 {
    match operation {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                println!("Ошибка: деление на ноль!");
                0.0
            } else {
                a / b
            }
        }
        _ => {
            println!("Неизвестная операция: {}", operation);
            0.0
        }
    }
}

fn main() {
    let a = 10.0;
    let b = 5.0;
    
    println!("{} + {} = {}", a, b, calculate(a, b, "+"));
    println!("{} - {} = {}", a, b, calculate(a, b, "-"));
    println!("{} * {} = {}", a, b, calculate(a, b, "*"));
    println!("{} / {} = {}", a, b, calculate(a, b, "/"));
    println!("{} % {} = {}", a, b, calculate(a, b, "%")); 
}