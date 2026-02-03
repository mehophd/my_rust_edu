fn multiply(x: i32, y: i32) -> i32 {
    x * y // последнее без точки с запятой возвращается
}

fn say_hello(name: &str) { // без возврата (возвращает неявно ()) ну нужно указывать тип возвращаемых данных
    println!("Hello, {}!", name);
} 

fn test_return(x: i32) -> bool {
    return x >= 0;
}

fn main() {
    let result = multiply(5, 7);
    say_hello("mehophd");
    println!("Result: {}", result);
    println!("{} is {}", 100, test_return(100));
}
