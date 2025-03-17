fn main() {
    let mut rng = rand::thread_rng();
    let secret_code: u8 = rng.gen_range(0..10);
    println!("The secret code is {}", secret_code);
}
