struct Rectangle {
    width: u32,
    height: u32,
}

// блок реализации
impl Rectangle {
    // метод &self (неизменяемое заимствование)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // метод &mut self (изменяемое заимствование)
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // ассоциированная функция (без self) - как конструктор
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect = Rectangle { width: 10, height: 5};
    println!("Площадь: {}", rect.area());

    let mut rect2 = Rectangle::square(3);
    rect2.scale(2);
    println!("Размеры: {}X{}", rect2.width, rect2.height);
}
