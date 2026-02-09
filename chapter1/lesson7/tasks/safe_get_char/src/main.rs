fn get_char_at(text: &str, index: usize) -> Option<char> {
    let result = text.chars().nth(index);

    match result {
        None => None,
        _ => result,
    }
}

fn main() {
    let text = "Rust";
    println!("{:?}", get_char_at(text, 0)); // Some('R')
    println!("{:?}", get_char_at(text, 5)); // None
}
