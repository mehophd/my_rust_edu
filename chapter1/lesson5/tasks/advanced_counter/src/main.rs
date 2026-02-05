struct Counter {
    value: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { value: 0 }
    }

    fn increment(&mut self) -> &mut Self {
        self.value += 1;
        self
    }

    fn add(&mut self, amount: u32) -> &mut Self {
        self.value += amount;
        self
    }

    fn get(&self) -> u32 {
        self.value
    }

    fn reset(&mut self) -> &mut Self {
        self.value = 0;
        self
    }
}

fn main() {
    let mut counter = Counter::new();
    counter.increment().increment().add(5).reset().increment();
    println!("Итог: {}", counter.get()); // 1
}