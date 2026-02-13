fn sum_and_average(vec: &Vec<i32>) -> (i32, f64) {
    if vec.len() == 0 {
        return (0, 0.0);
    }
    let mut sum: i32 = 0; 

    for i in 0..vec.len() {
        sum += vec[i];
    }

    let average: f64 = sum as f64 / vec.len() as f64;

    return (sum, average);
}

fn main() {
    let vec1 = vec![1, 2, 3]; 
    println!("{:?}", sum_and_average(&vec1));

    let vec2: Vec<i32> = Vec::new();
    println!("{:?}", sum_and_average(&vec2));
}
