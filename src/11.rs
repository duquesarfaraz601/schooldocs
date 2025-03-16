import randint

def get_random_rust_code():
    # Rust code to reverse a string using iterators
    let str = "Hello World!";
    let mut reversed = String::new();
    for c in str.chars().rev() {
        reversed.push(c);
    }
    return reversed;