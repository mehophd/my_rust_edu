fn validate_password(password: &str) -> Result<(), String> {
    let mut digit_flag = false;
    for v in password.chars() {
        if v.is_ascii_digit() {
            digit_flag = true;
            break;
        }
    }

    if password.len() >= 8 && password != password.to_lowercase() && digit_flag {
        Ok(())
    } else {
        let mut error_message = String::from("Пароль невалиден:\n");
        if password.len() < 8 {
            error_message += &String::from("Пароль слишком короткий\n");
        } 
        if password == password.to_lowercase() {
            error_message += &String::from("Добавьте хотя бы одну заглавную букву\n");
        } 
        if !digit_flag {
            error_message += &String::from("Добавьте хотя бы одну цифру\n");
        }
        Err(error_message)
    }
}

fn format_validate_password(password: &str) {
    match validate_password(password) {
        Ok(()) => println!("Пароль валиден\n"),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    format_validate_password("Password123");
    format_validate_password("pass");
    format_validate_password("pass11111111");
    format_validate_password("hehehaha");
}
