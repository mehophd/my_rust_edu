fn string_stats(s: &str) -> (usize, usize, usize) {
    let mut l = 0;
    let mut b = 0;
    let mut d = 0;

    for c in s.chars() {
        l += 1;
        b = b + if c.is_uppercase() {1} else {0};
        d = d + if c.is_digit(10) {1} else {0};
    }

    (l, b, d)
}

fn main() {
    println!("{:?}", string_stats("Hello, World! 42")); // (16, 2, 2)
    println!("{:?}", string_stats("Здравствуй, Мир! 42")); // (19, 2, 2)
}
