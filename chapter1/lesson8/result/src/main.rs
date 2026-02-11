fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Деление на ноль"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    
    match result {
        Ok(value) => println!("Результат: {}", value),
        Err(error) => println!("Ошибка: {}", error),
    }
}