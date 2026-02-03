// 1. Функция проверки чётности
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// 2. Функция вычисления факториала (рекурсивно или через цикл)
fn factorial(n: u32) -> u64 {
    let mut n: u64 = n as u64;
    let mut result: u64 = 1; 
    while n != 1 {
        result *= n;
        n -= 1;
    }
    return result;

    // альтернативный вариант
    // let mut result: u64 = 1;
    // for i in 1..=n {
    //     result *= i as u64;
    // }
    /  result
}

// 3. Функция определения знака числа
fn sign(n: i32) -> i32 {
    // Вернуть: 1 если >0, -1 если <0, 0 если ==0
    if n > 0 {
        return 1;
    } else if n < 0 {
        return -1;
    } else {
        return 0;
    }
}

fn main() {
    println!("4 чётное? {}", is_even(4));        // true
    println!("5 чётное? {}", is_even(5));        // false
    
    println!("5! = {}", factorial(5));           // 120
    
    println!("sign(10) = {}", sign(10));         // 1
    println!("sign(-5) = {}", sign(-5));         // -1
    println!("sign(0) = {}", sign(0));           // 0
}