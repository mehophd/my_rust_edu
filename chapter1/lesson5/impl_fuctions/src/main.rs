struct Counter {
    value: u32,
}

impl Counter {
    // &self — только читаем
    fn get(&self) -> u32 {
        self.value
}

    // &mut self — изменяем
    fn increment(&mut self) {
        self.value += 1;
}

// self — забираем владение
    fn into_inner(self) -> u32 {
        self.value  // После этого объект уничтожен
    }
}


// Ассоциированные функции
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
// Ассоциированная функция — конструктор
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

// Метод — работает с экземпляром
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(side: u32) -> Rectangle {
        let width = side;
        let height = side;
        Rectangle { width, height }
    }
}

//Цепочки вызовов
struct EmailBuilder {
    to: String,
    subject: String,
}

impl EmailBuilder {
    fn new() -> EmailBuilder {
        EmailBuilder {
            to: String::new(), // Создание пустой строки
            subject: String::new(),
        }
    }

    fn to(mut self, address: String) -> Self {
        self.to = address;
        self  // Возвращаем владение для продолжения цепочки
    }

    fn subject(mut self, text: String) -> Self {
        self.subject = text;
        self
    }

    fn send(self) {
        println!("Отправлено на {} с темой '{}'", self.to, self.subject);
    }
}

fn main() {
    let rect = Rectangle::new(10, 5);  // ::new — ассоциированная функция
    let square = Rectangle::square(8);
    println!("{}", rect.area());       // .area() — метод
    println!("{}", square.area());

    
    EmailBuilder::new()
    .to(String::from("user@example.com"))
    .subject(String::from("Привет!"))
    .send();
}