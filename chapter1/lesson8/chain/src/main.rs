fn step1() -> Result<i32, String> {
    Ok(42)
}

fn step2(x: i32) -> Result<i32, String> {
    if x > 0 {
        Ok(x * 2)
    } else {
        Err(String::from("Отрицательное число"))
    }
}

fn main() {
    let result1 = step1();
    
    match result1 {
        Ok(value1) => {
            let result2 = step2(value1);
            match result2 {
                Ok(value2) => println!("Результат: {}", value2),
                Err(e) => println!("Ошибка в шаге 2: {}", e),
            }
        },
        Err(e) => println!("Ошибка в шаге 1: {}", e),
    }
}