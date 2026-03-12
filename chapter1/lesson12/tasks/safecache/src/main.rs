mod cache;
use cache::SafeCache;

fn main() {
    let mut cache = SafeCache::new();
    println!("Кэш:");
    cache.print(); // Пустой вывод
    cache.set("key1".to_string(), "value1".to_string());
    cache.set("key2".to_string(), "value2".to_string());
    cache.print();

    match cache.get("key1") {
        Some(value) => {
            println!("Значение для key1: {}", value);
        }
        None => { println!("key1 не найден"); }
    }

    match cache.get("key3") {
        Some(value) => {
            println!("Значение для key3: {}", value);
        }
        None => { println!("key3 не найден"); }
    }

    //cache.data.clear(); // private field error
    let removed = cache.remove("key1");
    println!("{:?}", removed);
    cache.print();
    cache.clear();

    // Вывод:
    //  Кэш:
    // (key1, value1)
    // (key2, value2)
    // Значение для key1: value1
    // key3 не найден
    // Some("value1")
    // (key2, value2)
}
