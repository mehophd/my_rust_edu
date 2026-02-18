use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let mut result = text.to_string();

    for i in ['.', ',', '!', '?', ':', ';', '\'', '"'] {
        result = result.replace(i, " ");
    }

    for word in result.split_whitespace() {
        let word = word.to_lowercase();

        match map.get(&word) {
            Some(count) => map.insert(word.to_string(), count + 1),
            None => map.insert(word.to_string(), 1),
        };
    }

    map
}

fn main() {
    let map = word_count("hello world hello");

    println!("{:?}", map); // {"world": 1, "hello": 2}

    let map = word_count("Автостоп по фазу сна я всех вас ненавижу, суки, мама я сияю, мама я сияю, мама я сияю");

    println!("{:?}", map); // {"фазу": 1, "автостоп": 1, "вас": 1, "по": 1, "суки": 1, "мама": 3, "всех": 1, "я": 4, "ненавижу": 1, "сияю": 3, "сна": 1}
}
