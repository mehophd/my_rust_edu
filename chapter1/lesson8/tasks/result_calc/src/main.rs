fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b != 0.0 {
        Ok(a / b)
    } else {
        Err(format!("Деление на ноль. a = {}, b = {}", a, b))
    }
}

fn format_safe_divide(a: f64, b: f64) {
    match safe_divide(a, b) {
        Ok(res) => println!("Результат: {}", res),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    format_safe_divide(10.0, 2.0);
    format_safe_divide(10.0, 0.0);
}
