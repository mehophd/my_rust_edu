// Абсолютные пути от корня до крейта
mod network {
    pub mod tcp {
        pub fn connect() {}
    }
}

fn main() {
    crate::network::tcp::connect();  // абсолютный путь
}

// Относительные пути

mod network {
    pub fn helper() {}
    
    mod tcp {
        fn connect() {
            super::helper();  // обращение к родительскому модулю
            crate::network::helper();  // абсолютный путь тоже работает
        }
    }
}

/*
    crate:: — от корня текущего крейта
    self:: — текущий модуль (редко нужно, но полезно для избежания конфликтов имен)
    super:: — родительский модуль
*/