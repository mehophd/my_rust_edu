fn main() {
    let s: String = String::from("hello");
    let slice: &str = &s[0..2]; // "he"

    let s = "Привет";
    // Длина в байтах: 12 (каждая русская буква занимает 2 байта)
    // Количество символов: 6

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 перемещается, s2 остаётся
    println!("{}", s3);

    let s = "Привет";
    for c in s.chars() {
        println!("{}", c);
    }
}
