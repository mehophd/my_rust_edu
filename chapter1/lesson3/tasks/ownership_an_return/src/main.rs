fn concatenate(s1: String, s2: String) -> String {
    let result = s1 + &s2;
    result
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    println!("{}", concatenate(s1, s2));
    //попытка использования s1 и s2 вызовет ошибку из-за передачи владения
}
