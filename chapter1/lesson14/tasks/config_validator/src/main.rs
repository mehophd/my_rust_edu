trait FieldValidator {
    fn validate(&self, value: &str) -> Result<(), String>;
    fn name(&self) -> &str;
}

struct Config {
    port: String,
    timeout: String,
    username: String,
}

struct RequiredValidator;
struct NumericValidator;
struct LengthValidator {
    min: usize,
    max: usize
}

impl FieldValidator for RequiredValidator {
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.replace(" ", "").chars().count() > 0 {
            Ok(())
        } else {
            Err(format!("Значение \"{}\" пустое!", value))
        }
    }
    fn name(&self) -> &str { // Зачем нужны были эти методы? Они не используются
        "RequiredValidator"
    }
}

impl FieldValidator for NumericValidator {
    fn validate(&self, value: &str) -> Result<(), String> {
        let mut flag: bool = true;

        for i in 0..value.chars().count() {
            match value.chars().nth(i) {
                Some(c) => {
                    if !c.is_ascii_digit() {
                        flag = false;
                        break;
                    }
                }
                None => {}
            }
        }

        if flag {
            Ok(())
        } else {
            Err(format!("Значение \"{}\" содержит не только цифры!", value))
        }
    }
    fn name(&self) -> &str {
        "NumericValidator"
    }
}

impl FieldValidator for LengthValidator {
    fn validate(&self, value: &str) -> Result<(), String> { //Зачем тут нужен был key? Он никогда не ииспользовался
        if value.chars().count() >= self.min && value.chars().count() <= self.max {
            Ok(())
        } else {
            Err(format!("Значение \"{}\" не в нужных пределах!", value))
        }
    }
    fn name(&self) -> &str {
        "LengthValidator"
    }
}

impl Config {
    fn validate(&self, min: usize, max: usize) -> Result<(), Vec<String>> {
        let mut res: Vec<String> = Vec::new();
        let required = RequiredValidator {};
        let numeric = NumericValidator {};
        let length = LengthValidator { min: min, max: max};
        let mut flag: bool = true;

        match required.validate(&self.port) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match required.validate(&self.timeout) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match required.validate(&self.username) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match numeric.validate(&self.port) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match numeric.validate(&self.timeout) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match numeric.validate(&self.username) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match length.validate(&self.port) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match length.validate(&self.timeout) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        match length.validate(&self.username) {
            Ok(()) => {},
            Err(e) => { res.push(e); flag = false; }
        }

        if flag {
            Ok(())
        } else {
            Err(res)
        }
    }
}


fn main() {
    let cfg = Config {
        port: "8080".to_string(),
        timeout: "abc".to_string(),
        username: "".to_string(),  
    };
    match cfg.validate(0, 100) {
        Ok(_) => println!("Конфиг валиден"),
        Err(errors) => errors.iter().for_each(|e| println!("Ошибка: {}", e)), // я не знаю итераторы, использовал твой пример использования
    }

    // Ошибка: Значение "" пустое!
    // Ошибка: Значение "abc" содержит не только цифры!
    
}

// UPD: Изначально при составлении задача нейронка предполагала, 
// Что я догадаюсь, что не нужно применять каждый валидатор к каждому полю
// port: должно быть числом 1-65535 (обязательное + числовое + длина 1-5)
// timeout: должно быть числом 0-300 (обязательное + числовое + длина 1-3)
// username: может быть любым непустым текстом (обязательное + длина 1-20)