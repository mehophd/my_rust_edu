// ПОСЛЕ ИСПРАВЛЕНИЯ
struct Book {
    title: String,
    author: String,
    year: u16,
}

fn print_book(book: &Book) {
    println!("Книга: \"{}\" ({}, {})", book.title, book.author, book.year);
}

fn transfer_ownership(mut book: Book) -> Book {
    book.title.push_str(" (прочитано)");
    book
}

fn main() {
    let mut crime_and_punishment = Book {
        title: String::from("Преступление и наказание"),
        author: String::from("Достоевский"),
        year: 1866
    };
    print_book(&crime_and_punishment);
    print_book(&crime_and_punishment);
    print_book(&crime_and_punishment);
    crime_and_punishment = transfer_ownership(crime_and_punishment);
    print_book(&crime_and_punishment);
}
