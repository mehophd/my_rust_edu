struct ScientificCalculator{
    memory: f64,
    last_operation: String,
}

impl ScientificCalculator {
    fn new() -> Self {
        Self {
            memory: 0.0,
            last_operation: String::new()
        }
    }

    fn add(&mut self, x: f64) -> &mut Self {
        self.memory += x;
        self.last_operation = format!("add {}", x);
        self
    }

    fn subtract(&mut self, x: f64) -> &mut Self {
        self.memory -= x;
        self.last_operation = format!("subtract {}", x);
        self
    }

    fn sqrt(&mut self) -> &mut Self {
        if self.memory < 0.0 {
            self.last_operation = String::from("sqrt attempted on negative");
        } else {
            self.memory = self.memory.sqrt();
            self.last_operation = String::from("sqrt");
        }
        self
    }

    fn pow(&mut self, exponent: f64) -> &mut Self {
        self.memory = self.memory.powf(exponent);
        self.last_operation = format!("pow {}", exponent);
        self
    }

    fn clear(&mut self) -> &mut Self {
        self.memory = 0.0;
        self.last_operation = String::from("clear");
        self
    }

    fn show(&self) -> f64 {
        self.memory
    }

    fn print_last_operation(&self) {
        println!("{}", self.last_operation);
    }
}

fn main() {
    let mut calc = ScientificCalculator::new();
    calc.add(10.0).sqrt().pow(2.0);
    println!("{}", calc.show()); // должно вернуть 10.0
    calc.print_last_operation(); // показывает "pow 2.0"
}