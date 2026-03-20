struct BorrowedCache<'a, 'b> {
    key: &'a str,
    value: &'b str,
}

impl<'a, 'b> BorrowedCache<'a, 'b> {
    fn new(key: &'a str, value: &'b str) -> Self {
        Self {key: key, value: value}
    }

    fn get_key(&self) -> &'a str {
        self.key
    }
}

fn main() {
    let long_key = String::from("long_lived_key");  // живёт до конца main
    
    {
        let short_value = String::from("short");  // живёт только в блоке
        
        // С ДВУМЯ циклами — КОМПИЛИРУЕТСЯ!
        let cache = BorrowedCache::new(&long_key, &short_value);
        println!("{}", cache.get_key());  // OK: cache живёт столько же, сколько short_value
    }  // short_value умирает → cache тоже умирает → безопасно
    
    println!("{}", long_key);  // long_key всё ещё жив
}
