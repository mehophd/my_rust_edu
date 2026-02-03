fn my_cmp(first_num: i32, second_num: i32) -> i32 {
    let range = first_num - second_num;
    if range > 0 {
        return 1;
    } else if range < 0 {
        return -1;
    } else {
        return 0;
    }
}

fn check_guess(guess: i32, secret: i32) -> String {
    match my_cmp(guess, secret) {
        1 => String::from("Слишком много!"),
        -1 => String::from("Слишком мало!"),
        0 => String::from("Точно!"),
        _ => String::from("Костыль, rust ругается без него, т.к. у i32 очень много значений"),
    } 
}

// более безопасный вариант через ordering
//fn check_guess(guess: i32, secret: i32) -> String {
//    match guess.cmp(&secret) {
//        std::cmp::Ordering::Less => String::from("Слишком мало!"),
//        std::cmp::Ordering::Greater => String::from("Слишком много!"),
//        std::cmp::Ordering::Equal => String::from("Точно!"),
//    }
//}

fn main() {
    let secret_number = 42;
    
    println!("Угадай число (попытки):");
    
    for guess in [10, 50, 42, 30].iter() {
        let result = check_guess(*guess, secret_number);
        println!("{} -> {}", guess, result);
    }
}