fn main() {
    let mut count = 0; // mut означает, что переменная изменяемая

    loop {
        count += 1;

        if count == 3 {
            println!("Три");
            continue;
        }

        println!("Счет: {}", count);

        if count == 5 { break };

    }

    let result = loop {
        count += 1;
        if count > 10 {
            break count * 2; // значение после break возвращается
        }
    };

    println!("Результат: {}", result);


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Поехали!");

    for number in 1..4 {
        println!("{}", number);
    }

    for number in 1..=4 { // включительно справа
        println!("{}", number);
    }

    // итерация по строке
    for c in "rust".chars() {
        println!("{}", c);
    }
}
