fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(s_int) => Ok(s_int),
        Err(_) => Err("Не число либо не вмещается в i32".to_string()),
    }
}


fn validate_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err("Число нечетное".to_string())
    }
}


fn double_number(n: i32) -> Result<i32, String> {
    match n.checked_mul(2) {
        Some(a) => Ok(a),
        _ => Err("Число слишком большое".to_string())
    }
}


fn process_string(s: &str) -> Result<i32, String> {
    match parse_number(s) {
        Ok(parsed_n) => {
            match validate_even(parsed_n) {
                Ok(even_n) => {
                    match double_number(even_n) {
                        Ok(even_n) => Ok(even_n),
                        Err(e) => Err(e)
                    }
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}


fn main() {
    println!("{:?}", process_string("42")); // Ok(84)
    println!("{:?}", process_string("41")); // Err("Число нечетное")
    println!("{:?}", process_string("41a")); // Err("Не число либо не вмещается в i32")
    println!("{:?}", process_string("5555555555555555555555555555555555")); //Err("Не число либо не вмещается в i32")
}
