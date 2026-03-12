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
        let mut msg = String::new();

        if password.chars().count() < 12 {
            msg += "пароль слишком короткий; ";
        }
        if !digit {
            msg += "нету цифры; ";
        }
        if !special_symbol {
            msg += "нет спецсимвола; ";
        }
        if !case {
            msg += "нет заглавной буквы; ";
        }

        Err(msg[0..2].to_uppercase().to_string() + &msg[2..msg.len()-2])

    }
}

pub fn is_correct_password(password: &str) -> bool {
    match validate_strong_password(password) {
        Ok(_) => true,
        Err(code) => { println!("{}", code); false }
    }
}
