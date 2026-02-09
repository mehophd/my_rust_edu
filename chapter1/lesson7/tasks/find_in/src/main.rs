fn find_in_list(list: &[i32], target: i32) -> Option<usize> {
    for i in 0..list.len() {
        if list[i] == target {
            return Some(i);
        } 
    } None
}

fn main() {
    let numbers = [10, 20, 30, 40, 50];
    println!("{:?}", find_in_list(&numbers, 30)); // Some(2)
    println!("{:?}", find_in_list(&numbers, 99)); // None
}
