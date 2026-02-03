fn trim_and_uppercase(s: &mut String) {
    // 1. Убираем пробелы и получаем &str
    let trimmed = s.trim();
    
    // 2. Преобразуем в верхний регистр и получаем новый String
    let upper = trimmed.to_uppercase();
    
    // 3. Заменяем содержимое исходной строки
    *s = upper;
}

fn main() {
    let mut text = String::from("  Rust - крутой язык!  ");
    trim_and_uppercase(&mut text);
}
