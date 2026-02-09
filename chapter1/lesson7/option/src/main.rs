fn find_divisor(n: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(n / divisor)
    }
}

fn main() {
    let result = find_divisor(10, 2);
    
    // Обработка с помощью match (обязательно обработать оба случая!)
    match result {
        Some(value) => println!("Результат: {}", value),
        None => println!("Деление на ноль!"),
    }

    // Автоматический вывод типа
    let some_number = Some(5);        // Option<i32>
    let some_string = Some("hello");  // Option<&str>

    // Для None нужно указывать тип явно
    let absent_number: Option<i32> = None;
    let absent_string: Option<&str> = None;

    let x = Some(5);
    // let y = x + 1; // ОШИБКА! Нельзя использовать Option напрямую
} 