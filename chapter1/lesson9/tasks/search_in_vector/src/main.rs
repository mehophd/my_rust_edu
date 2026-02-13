fn find_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
    let mut index = None;
    for i in 0..vec.len() {
        if vec[i] == target {
            index = Some(i);
            break;
        }
    }

    index
}

fn main() {
    let vec = vec![2, 5, 10, 4, 4, 5, 4];
    let target1: i32 = 4;
    let target2: i32 = 7;

    println!("{:?}", find_index(&vec, target1)); // Some(3)
    println!("{:?}", find_index(&vec, target2)); // None
}
