struct UserProfile {
    username: String,
    email: String,
    posts_count: u32,
}

impl UserProfile {
    fn new(username: String, email: String) -> UserProfile {
        UserProfile {
            username: username,
            email: email,
            posts_count: 0
        }
    }

    fn post_message(&mut self) {
        self.posts_count += 1;
    }

    fn get_info(&self) -> String {
        format!("{} (<{}>), посты: {}", self.username, self.email, self.posts_count)
    }
}

fn main() {
    let mut rustacean = UserProfile::new(String::from("rustacean"), String::from("rustacean@example.com"));

    println!("{}", rustacean.get_info());
    rustacean.post_message();
    rustacean.post_message();
    rustacean.post_message();
    println!("{}", rustacean.get_info());
}
