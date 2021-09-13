use std::io::stdin;

fn what_is_your_name() -> String {          // <callout id="co.function" />
    let mut your_name = String::new();      // <callout id="co.readline" />
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name                               // <callout id="co.return" />
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name(); // <callout id="co.call" />
    println!("Hello, {}", name);
}
