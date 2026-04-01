trait Drawable {
    // Обязательный метод (без реализации по умолчанию)
    fn draw(&self);
    
    // Необязательный метод с реализацией по умолчанию
    fn clear(&self) {
        println!("Очистка экрана");
    }
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Рисую круг радиусом {}", self.radius);
    }

    //clear() используем по умолчанию
}

fn main() {
    let c = Circle { radius: 5.0 };
    c.draw();
    c.clear();
}

// struct MyType;
// trait MyTrait;

// impl MyTrait for MyType {}  // OK

// // Но нельзя:
// impl Display for MyType {}  // OK (Display из стандартной библиотеки)
// impl MyTrait for String {}  // OK (String из стандартной библиотеки)
// // impl Display for String {}  // ЗАПРЕЩЕНО! (оба извне крейта)