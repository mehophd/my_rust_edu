use std::collections::HashMap;

fn invert_map(original: &HashMap<String, i32>) -> HashMap<i32, String> {
    let mut map: HashMap<i32, String> = HashMap::new();

    for (value, key) in original {
        map.insert(*key, value.to_string());
    }

    map
}

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    map.insert("три".to_string(), 3);
    map.insert("пять".to_string(), 5);

    println!("{:?}", map); // {"три": 3, "пять": 5}
    println!("{:?}", invert_map(&map)); // {3: "три", 5: "пять"}
}
