fn censor(text: &str, bad_words: &[&str]) -> String {
    let mut result = text.to_string();
    for i in bad_words {
        result = result.replace(i, "***");
    }

    result.to_string()
}

fn main() {
    println!("{}", censor("hello world", &["hello"]));
}
