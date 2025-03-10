// Rust Code Snippet
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // filter out odd numbers
    v.retain(|num| num % 2 == 0);

    println!("Filtered vector: {:?}", v);
}
