// START: stdin
use std::io::stdin;
// END: stdin

fn main() {
    // START: hello
    println!("Hello, what's your name?");
    // END: hello
    // START: variable
    let mut your_name = String::new();
    // END: variable
    // START: readline
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    // END: readline
    // START: greet
    println!("Hello, {}", your_name)
    // END: greet
}
