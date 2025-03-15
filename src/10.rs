fn main() {
    let mut numbers = vec![1, 2, 3];
    let num_to_find = 4;
    if numbers.contains(&num_to_find) {
        println!("Found number {}", num_to_find);
    } else {
        println!("Number not found");
    }
}
