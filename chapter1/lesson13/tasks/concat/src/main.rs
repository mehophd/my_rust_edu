fn concat_with_space<'a>(s1: &'a str, s2: &'a str) -> String {
    s1.to_owned() + " " + s2
}

fn main() {
    println!("{}", concat_with_space("Hello,", "world!"));
    // так как возвращается владение, то указывать цикл не нужно
}
