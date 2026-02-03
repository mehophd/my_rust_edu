struct Book {
    title: String,   // Book владеет строкой title
    author: String,  // Book владеет строкой author
}

fn main() {
    let book = Book {
        title: String::from("1984"),
        author: String::from("Orwell"),
    };
    
    // При выходе из области видимости:
    // 1. Вызывается drop() для book.title
    // 2. Вызывается drop() для book.author
    // 3. Память освобождается
}