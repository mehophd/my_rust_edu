fn create_and_modify() {
    let mut v: Vec<i32> = Vec::new();
    for i in 1..=5 {
        v.push(i);
    }

    let last = v.pop();
    match last {
        Some(a) => println!("{}", a),
        None => println!("Никто не пришел"),
    }

    println!("{:?}", v);
}

fn main() {
    create_and_modify();
}
