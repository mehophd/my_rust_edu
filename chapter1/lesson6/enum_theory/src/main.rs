// Варианты могут содержать данные
enum Message {
    Quit,                       // Без данных
    Move { x: i32, y: i32 },    // С анонимной структурой
    Write(String),              // С одним значением
    ChangeColor(i32, i32, i32), // С кортежем
}

fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Выход"),
        Message::Move { x, y } => println!("Перемещение в ({}, {})", x, y),
        Message::Write(text) => println!("Текст: {}", text),
        Message::ChangeColor(r, g, b) => println!("Цвет: RGB({}, {}, {})", r, g, b),
    }
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(text) => println!("Вызываю с текстом: {}", text),
            _ => println!("Другой тип сообщения"),
        }
    }
}


fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Привет"));

    msg3.call();
    handle_message(msg3);
}
