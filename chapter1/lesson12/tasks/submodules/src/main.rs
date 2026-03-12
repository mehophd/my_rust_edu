mod math {
    pub mod stats {
        pub fn average(numbers: &[i32]) -> f64 {
            if numbers.is_empty() {
                return 0.0;
            }

            super::sum(numbers) as f64 / numbers.len() as f64
        }
    }

    fn sum(numbers: &[i32]) -> i32 {
        let mut result: i32 = 0;

        for n in numbers {
            result += *n;
        }

        result
    }
}

fn main() {
    println!("{:?}", math::stats::average(&[1, 2, 3])); // 2.0
}
