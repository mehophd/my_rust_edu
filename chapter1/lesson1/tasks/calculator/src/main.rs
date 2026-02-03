fn main() {
    let a: i32 = 42;
    let b: i32 = 17;

    println!("Числа: a = {}, b = {}", a, b);
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);


    let x: f64 = 3.14;
    let y: f64 = 2.0;

    println!("\nЧисла: x = {}, y = {}", x, y);
    println!("x + y = {}", x + y);
    println!("x * y = {}", x * y);
    println!("x / y = {}", x / y);

    let radius: f64 = 5.0;
    let pi: f64 = 3.14159;
    let area = pi * radius * radius;

    println!("\nПлощадь круга с радиусом {} = {:.2}", radius, area);
}
