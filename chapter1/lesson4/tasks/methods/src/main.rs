struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height || rect.height < self.width && rect.width < self.height
    }

    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
}

fn main() {
    let rect1 = Rectangle::new(5, 10);
    let rect2 = Rectangle::new(4, 3);

    println!("Площадь прямоугольника 1: {}", rect1.area());
    println!("Площадь прямоугольника 2: {}", rect2.area());
    println!("Прямоугольник 1 может вместить прямоугольник 2: {}", rect1.can_hold(&rect2));
}