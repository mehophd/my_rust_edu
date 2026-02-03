fn print_length(s: &String) {
    println!("{}", s.len());
}

fn main() {
    let my_string = String::from("привет");
    print_length(&my_string);

    println!("{}", my_string); // теперь не вызовет благодаря заимствованию
}
