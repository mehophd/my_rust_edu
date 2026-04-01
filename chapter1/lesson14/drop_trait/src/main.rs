struct File {
    name: String,
}

impl Drop for File {
    fn drop(&mut self) {
        println!("Закрываю файл: {}", self.name);
        // Здесь могла бы быть системная операция close()
    }
}

fn main() {
    let f = File { name: String::from("data.txt") };
    println!("Работаю с файлом...");
}  // при выходе из области видимости автоматически вызывается drop()
// Вывод:
// Работаю с файлом...
// Закрываю файл: data.txt