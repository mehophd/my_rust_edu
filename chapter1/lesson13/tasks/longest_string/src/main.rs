fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.chars().count() > s2.chars().count() {
        s1
    } else {
        s2
    }
}

fn main() {
    let a = "abcde";
    let b = "fghijklm";
    let c = "nopqr";
    println!("a: {} b: {} longest: {}", a, b, longest(a, b));
    println!("a: {} c: {} longest: {}", a, c, longest(a, c));
    println!("lit1: {} lit2: {} longest: {}", "hello", "world!", longest("hello", "world!"));

    //a: abcde b: fghijklm longest: fghijklm
    //a: abcde c: nopqr longest: nopqr
    //lit1: hello lit2: world! longest: world!
}
