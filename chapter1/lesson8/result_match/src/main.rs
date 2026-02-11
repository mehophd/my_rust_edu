fn read_file(path: &str) -> Result<String, String> {
    // Представим, что здесь чтение файла
    if path == "" {
        Err(String::from("Путь пустой"))
    } else {
        Ok(String::from("Содержимое файла"))
    }
}

fn main() {
    let content = read_file("data.txt");
    
    match content {
        Ok(text) => println!("Текст: {}", text),
        Err(e) => println!("Ошибка чтения файла: {}", e),
    }
}