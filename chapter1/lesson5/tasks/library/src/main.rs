struct LibraryBook {
    title: String,
    author: String,
    is_borrowed: bool,
    borrower_name: String,
}

impl LibraryBook {
    fn new(title: String, author: String) -> Self {
        Self {
            title: title,
            author: author,
            is_borrowed: false,
            borrower_name: String::new()
        }
    }

    fn borrow(&mut self, borrower: String) {
        if self.is_borrowed {
            println!("Книга уже занята: {}", self.borrower_name);
            return;
        }
        self.is_borrowed = true;
        self.borrower_name = borrower;
    }

    fn return_book(&mut self) -> String {
        if !self.is_borrowed {
            println!("Книга и так на полке");
            return String::new();
        }
        self.is_borrowed = false;
        self.borrower_name.clone()
    }

    fn get_status(&self) -> String {
        format!("Книга {} ({}) выдана: {}", self.title, self.author, self.borrower_name)
    }
}

fn main() {
    let mut book = LibraryBook::new(
        String::from("1984"), 
        String::from("Джордж Оруэлл")
    );
    
    book.borrow(String::from("Алиса"));
    println!("{}", book.get_status()); // "Книга '1984' (Оруэлл): выдана Алиса"
    
    book.borrow(String::from("Боб")); // "Книга уже занята: Алиса"
    
    let reader = book.return_book();
    println!("{} вернул книгу", reader); // "Алиса вернул книгу"
}