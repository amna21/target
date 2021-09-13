use std::io::stdin;

// START: derive
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String
}
// END: derive

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    // START: vecmacro
    let mut visitor_list = vec![
        Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("Steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("Fred", "Wow, who invited Fred?"),
    ];
    // END: vecmacro

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        // START: known_guest
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {    // <callout id="co.is_empty" />
                    break;  // <callout id="co.break" />
                } else {
                    println!("{} is not on the visitor list.", name);
                    //START_HIGHLIGHT
                    visitor_list.push(Visitor::new(&name, "New friend"));
                    //END_HIGHLIGHT
                }
            }
        }
        // END: known_guest
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
