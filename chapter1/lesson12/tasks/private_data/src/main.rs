mod password_checker;

mod user {
    use crate::password_checker;

    pub struct User {
        password_hash: String
    }
    
    impl User {
        pub fn new() -> Self {
            Self{ password_hash: String::new() }
        }
        
        pub fn get_password(&self) -> String {
            if self.password_hash.is_empty() {
                println!("Пароль не установлен");
            }
            self.password_hash.clone()
        }

        pub fn set_password(&mut self, raw: &str) {
            if !password_checker::is_correct_password(raw) {
                println!("Пароль некорректный");
                return;
            }
            self.password_hash = "hash_".to_string() + raw;
        }
    }
    
    fn verify_hash(s: &str) -> bool {
        match s.split_once('_') {
            Some((hash, _)) => hash == "hash",
            None => false,
        }
    }
}


fn main() {
    let mut user1 = user::User::new();
    user1.set_password("123");
    println!("\n");
    println!("Нынешний пароль: {}\n", user1.get_password());
    user1.set_password("Qrt/111444dfheg");
    println!("Нынешний пароль: {}\n", user1.get_password());

    // Вывод
    // Пароль слишком короткий; нет спецсимвола; нет заглавной буквы
    // Пароль некорректный
    //
    //
    // Пароль не установлен
    // Нынешний пароль:  
    //
    // Нынешний пароль: hash_Qrt/111444dfheg

}
