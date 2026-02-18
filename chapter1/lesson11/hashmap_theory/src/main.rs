use std::collections::HashMap;

fn main() {
    // Пустая хэш-мапа
    let mut map: HashMap<String, i32> = HashMap::new();

    // Вставка
    map.insert(String::from("key"), 10);
    map.insert(String::from("another"), 20);

    // Получение значения по ключу – возвращает Option<&V>
    let value = map.get("key"); // Some(&10)
    let missing = map.get("absent"); // None

    // Обновление значения – просто вставляем снова тот же ключ
    map.insert(String::from("key"), 30); // теперь по ключу "key" лежит 30

    // Проверка наличия ключа без извлечения значения
    if map.contains_key("key") {
        println!("Ключ существует");
    }

    let key = String::from("name");
    let value = String::from("Alice");
    let mut map = HashMap::new();
    map.insert(key, value); // key и value перемещены
    // println!("{}", key); // ОШИБКА!

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    for (key, value) in &map {
        println!("{}: {}", key, value);
}
}