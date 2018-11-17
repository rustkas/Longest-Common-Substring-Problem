extern crate find_longest_substring;
/*
cargo test
cargo build --examples
cargo run --example example1
cargo test --examples
cargo test --example example1
*/
use find_longest_substring::longest_substring;

fn main() {
    let input_string = "abacadaddaad";
    let longest_string = longest_substring(input_string);
    println!("The input string is '{}'", input_string);
    println!(
        "The longest string is '{}'. Length = {}",
        longest_string,
        longest_string.len()
    );
}
