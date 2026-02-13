fn add_if_not_exists(vec: &mut Vec<String>, item: String) -> Result<(), String> {
    for i in 0..vec.len() {
        if vec[i] == item {
            return Err("Элемент уже существует".to_string());
        }
    }
    vec.push(item);
    return Ok(());
}

fn main() {
    let mut vec: Vec<String> = Vec::new();
    vec.push("а".to_string());
    vec.push("б".to_string());
    vec.push("в".to_string());

    let vec2 = vec!["г", "с", "б", "г"];
    
    for i in 0..vec2.len() {
        match add_if_not_exists(&mut vec, vec2[i].to_string()) {
            Ok(()) => println!("{:?}", vec),
            Err(e) => println!("{}", e),
        }
    }
    /*
    ["а", "б", "в", "г"]
    ["а", "б", "в", "г", "с"]
    Элемент уже существует
    Элемент уже существует
    */
}
