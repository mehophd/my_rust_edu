struct Letter {
    sender: String,
    recipient: String,
    body: String,
}

impl Letter {
    fn new(sender: String) -> Letter {
        Letter { 
            sender: sender, 
            recipient: String::new(),
            body: String::new() 
        }
    }

    fn to(mut self, recipient: String) -> Self {
        self.recipient = recipient;
        self
    }

    fn body(mut self, body: String) -> Self {
        self.body = body;
        self
    }

    fn send(self) {
        println!("От: {}\nКому: {}\n---\n{}", self.sender, self.recipient, self.body);
    }
 }

 fn main() {
    Letter::new(String::from("Алиса"))
        .to(String::from("Боб"))
        .body(String::from("Привет! Как дела?"))
        .send();
 }