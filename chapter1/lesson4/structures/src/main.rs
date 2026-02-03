// Определение структуры
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User { // можно сделать изменяемой с mut
        username: String::from("mehophd"),
        email: String::from("mehophd@example.com"),
        sign_in_count: 1,
        active: true,
    };

    // Доступ к полям
    println!("Имя: {}", user1.username);
    user1.username = String::from("Adolf");
    println!("Имя: {}", user1.username);
}
