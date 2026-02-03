fn append_world(s: &mut String) {
    s.push_str(" мир");
    println!("{}", s);
}

fn main() {
    let mut string = String::from("здравствуй");
    append_world(&mut string);
    append_world(&string); // функция ожидает изменяемую ссылку
    append_world(&mut string);
}
