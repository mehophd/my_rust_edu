struct Person {
    name: String,
    age: u8,
}

fn introduce(person: &Person) {
    println!("Привет, меня зовут {}, мне {} лет", person.name, person.age);
}

fn birthday(person: &mut Person) {
    person.age += 1;
}

fn main() {
    let mut henry = Person { name: String::from("Генри"), age: 42};
    introduce(&henry);
    birthday(&mut henry);
    introduce(&henry);
}
