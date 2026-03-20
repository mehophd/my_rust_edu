struct UserProfile<'a> {
    username: &'a str,
}

impl<'a> UserProfile<'a> {
    fn greeting(&self) -> String {
        format!("Hello, {}!", self.username)
    }
}

fn main() {
    let user = UserProfile { username: "mehophd" };
    let result = user.greeting();

    println!("{}", result);

        let failed_user;
    {
        let fail = String::from("fail");
        failed_user = UserProfile { username: &fail };
    }
    failed_user.greeting();

    /*
    error[E0597]: `fail` does not live long enough
    --> src/main.rs:20:47
    |
    19 |         let fail = String::from("fail");
    |             ---- binding `fail` declared here
    20 |         failed_user = UserProfile { username: &fail };
    |                                               ^^^^^ borrowed value does not live long enough
    21 |     }
    |     - `fail` dropped here while still borrowed
    22 |     failed_user.greeting();
    |     ----------- borrow later used here

    For more information about this error, try `rustc --explain E0597`.
    */
}
