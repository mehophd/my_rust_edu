use std::collections::HashMap;

fn group_by_first_letter(words: Vec<String>) -> HashMap<char, Vec<String>> {
    let mut map: HashMap<char, Vec<String>> = HashMap::new();

    for word in words {
        let letter = match word.chars().next() {
            Some(c) => c,
            None => continue,
        };
        match map.get_mut(&letter) {
            Some(vector) => { vector.push(word); }
            None => { map.insert(letter, vec![word]); }
        };
    }

    map
}

fn main() {
    let words = vec!["apple".to_string(), "apricot".to_string(), "banana".to_string()];
    let map = group_by_first_letter(words);

    println!("{:?}", map); // {'b': ["banana"], 'a': ["apple", "apricot"]}
}
