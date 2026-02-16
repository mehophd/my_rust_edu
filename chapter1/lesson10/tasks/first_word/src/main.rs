fn first_word(s: &str) -> &str {
    let mut white: Option<usize> = None;

    for (id, ch) in s.char_indices() {
        if ch == ' ' {
            white = Some(id);
            break;
        }
    }

    match white {
        Some(id) => &s[0..id],
        None => &s,
    }
}

fn main() {
    println!("{}", first_word("What are you doing here?")); // What
    println!("{}", first_word("  HELLO")); //
    println!("{}", first_word("WORLD")); // WORLD
}
