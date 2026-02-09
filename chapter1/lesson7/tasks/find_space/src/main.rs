fn find_first_space(s: &str) -> Option<usize> {
    for (index, ch) in s.char_indices() {
        if ch == ' ' {
            return Some(index);
        }
    }
    None
}

fn main() {
    println!("{:?}", find_first_space("Hello, world!")); // Some(6)
    println!("{:?}", find_first_space("Hello!")); // None
}
