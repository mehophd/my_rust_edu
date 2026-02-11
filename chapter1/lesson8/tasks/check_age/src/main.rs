fn check_age(age: i16) -> Result<i16, String> {
    if age >= 0 && age <= 150 {
        Ok(age)
    } else if age < 0 {
        Err(String::from("Возраст не может быть отрицательным"))
    } else {
        Err(String::from("Возраст не может быть больше 150"))
    }
}

fn print_age(age: i16) {
    match check_age(age) {
        Ok(age) => println!("Возраст: {}", age),
        Err(e) => println!("Ошибка! {}", e),
    }
}

fn main() {
   print_age(25); // Возраст: 25
   print_age(-4); // Ошибка! Возраст не может быть отрицательным
   print_age(1488); // Ошибка! Возраст не может быть больше 150
}
