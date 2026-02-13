struct Student {
    name: String,
    grade: u8
}

fn honor_students(students: &Vec<Student>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for i in 0..students.len() {
        if students[i].grade == 5 {
            res.push(students[i].name.clone());
        }
    }

    res
}

fn main() {
    let mut vec = vec![
        Student { name: "Vasya".to_string(), grade: 5},
        Student { name: "Petya".to_string(), grade: 4},
        Student { name: "Vanya".to_string(), grade: 3},
        Student { name: "Misha".to_string(), grade: 5}
    ];

    println!("{:?}", honor_students(&vec)); // ["Vasya", "Misha"]
}
