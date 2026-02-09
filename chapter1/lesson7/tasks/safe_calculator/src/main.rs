struct SafeCalculator {
    a: i32,
    b: i32
}

impl SafeCalculator {
    fn divide(&self) -> Option<i32> {
        if self.b != 0 {
            Some(self.a / self.b)
        } else {
            None
        }
    }

    fn add(&self) -> Option<i32> {
        Some(self.a + self.b)
    }

    fn multiply(&self) -> Option<i32> {
        Some(self.a * self.b)
    }

    fn subtract(&self) -> Option<i32> {
        Some(self.a - self.b)
    }
}

fn main() {
    let calc = SafeCalculator {
        a: 2,
        b: 5
    };
    println!("Multiply: {:?}", calc.multiply()); // Multiply: Some(10)

    let calc = SafeCalculator {
        a: 2,
        b: 0
    };
    println!("Divide: {:?}", calc.divide()); // NDivide: None
}


/* Можно было и проще, только тогда не понятно нахрен по задаче здесь структура
struct SafeCalculator;

impl SafeCalculator {
    fn divide(a: i32, b: i32) -> Option<i32> {
        if b != 0 {
            Some(a / b)
        } else {
            None
        }
    }

    fn add(a: i32, b: i32) -> Option<i32> {
        Some(a + b)
    }

    fn multiply(a: i32, b: i32) -> Option<i32> {
        Some(a * b)
    }

    fn subtract(a: i32, b: i32) -> Option<i32> {
        Some(a - b)
    }
}

fn main() {
    println!("Multiply: {:?}", SafeCalculator::multiply(2, 5)); // Some(10)
    println!("Divide: {:?}", SafeCalculator::divide(2, 0));    // None
}
*/