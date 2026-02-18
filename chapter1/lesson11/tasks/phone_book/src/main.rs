use std::collections::HashMap;

struct Contact {
    name: String,
    phone: String
}


fn add_contact(book: &mut HashMap<String, Contact>, name: String, phone: String) {
    if book.contains_key(&name) {
        println!("Контакт {} уже существует", name);
    } else {
        book.insert(name.clone(), 
            Contact {
                name: name.clone(),
                phone: phone
            }
        );
        println!("Контакт {} успешно добавлен", name);
    }
}

fn get_phone(book: &HashMap<String, Contact>, name: &str) -> Option<String> {
    match book.get(name) {
        Some(contact) => Some(contact.phone.clone()),
        None => None,
    }
}


fn main() {
    let mut book: HashMap<String, Contact> = HashMap::new();
    add_contact(&mut book, "Vasya".to_string(), "88005553535".to_string()); // Контакт Vasya успешно добавлен
    add_contact(&mut book, "Vasya".to_string(), "88005553535".to_string()); // Контакт Vasya уже существует

    println!("{:?}", get_phone(&book, "Vasya")); // Some("88005553535")
    println!("{:?}", get_phone(&book, "Petya")); // None
}
