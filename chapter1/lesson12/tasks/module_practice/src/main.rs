mod config {
    pub mod parser {
        use super::validator::validate_key;

        pub fn parse_line(line: &str) -> Result<(String, String), String> {
            match line.split_once('=') {
                Some((key, value)) => {
                    let key = key.trim();
                    let value = value.trim();

                    if key.is_empty() || !validate_key(key) {
                        return Err("Ошибка, некорректный ключ".to_string());
                    }

                    if value.is_empty() {
                        return Err("Ошибка, некорректное значение".to_string());
                    }

                    Ok((key.to_string(), value.to_string()))
                }
                None => Err("Ошибка, некорректный ввод".to_string())
            }
        }
    }

    mod validator {
        pub fn validate_key(key: &str) -> bool {
            for ch in key.chars() {
                if "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890".contains(ch) {
                    continue;
                } else {
                    return false;
                }
            }
            return true;
        }
    }
}

fn main() {
    println!("{:?}", config::parser::parse_line("host=localhost"));
    println!("{:?}", config::parser::parse_line("port=8080"));

    println!("{:?}", config::parser::parse_line("host"));
    println!("{:?}", config::parser::parse_line("host="));      
    println!("{:?}", config::parser::parse_line("=localhost"));  
    println!("{:?}", config::parser::parse_line("host@=value")); 
}
