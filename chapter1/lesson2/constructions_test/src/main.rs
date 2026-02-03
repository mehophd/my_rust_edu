fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 }; // условие в переменной

    println!("Значение {}", number);

    let a: i32 = 10;
    let b: i32 = 10;

    if a * b > 100 {
        println!("Больше 100");
    } else if a * b < 100 {
        println!("Меньше 100");
    }
    else {
        println!("Ровно 100");
    }

    // Типы веток ДОЛЖНЫ совпадать!
    // let bad = if condition { 5 } else { "шесть" }; // ОШИБКА!
}
