fn validate_strong_password(password: &str) -> Result<(), String> {
    let mut digit = false;
    let mut special_symbol = false;
    let case = password != password.to_lowercase();

    for ch in password.chars() {
        if ch.is_digit(10) {
            digit = true;
        }
        if "!@#$%^&*()_+-=[]{}|;:,.<>?/".contains(ch) {
            special_symbol = true;
        } 
    }

    if password.chars().count() >= 12 && digit && special_symbol && case {
        Ok(())
    } else {
        let mut msg: Vec<&str> = Vec::new();

        if password.chars().count() < 12 {
            msg.push("пароль слишком короткий");
        }
        if !digit {
            msg.push("нету цифры");
        }
        if !special_symbol {
            msg.push("нет спецсимвола");
        }
        if !case {
            msg.push("нет заглавной буквы");
        }

        let combined = msg.join("; ");
        let result = format!("{}{}", 
            combined[..2].to_uppercase(), 
            &combined[2..]
        );
        Err(result)
    }
}

fn main() {
    println!("{:?}", validate_strong_password("ffg")); // Err("Пароль слишком короткий; нету цифры; нет спецсимвола; нет заглавной буквы")
    println!("{:?}", validate_strong_password("abobusabobus")); // Err("Нету цифры; нет спецсимвола; нет заглавной буквы")
    println!("{:?}", validate_strong_password("abobusabobus1488")); // Err("Нет спецсимвола; нет заглавной буквы")
    println!("{:?}", validate_strong_password("abObUsabobus1488")); // Err("Нет спецсимвола")
    println!("{:?}", validate_strong_password("@bObUsabobus1488")); // Ok(())
}
