use std::io::stdin;

// START: struct
struct Visitor {
    name: String,
    greeting: String,
}
// END: struct

// START: impl
impl Visitor {    // <callout id="co.impl" />
    fn new(name: &str, greeting: &str) -> Self {    // <callout id="co.fn" />
        Self {  // <callout id="co.makestruct" />
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) { // <callout id="co.greet" />
        println!("{}", self.greeting);
    }
}
//END: impl

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    // START: array
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve","Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];
    // END: array

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    //START: iter_find
    let known_visitor = visitor_list// <callout id="co.guestlist.assign" />
        .iter()// <callout id="co.guestlist.iter" />
        .find(|visitor| visitor.name == name);// <callout id="co.guestlist.find" />
    //END: iter_find
    //START: iter_match
    match known_visitor {// <callout id="co.guestlist.match_on" />
        Some(visitor) => visitor.greet_visitor(),// <callout id="co.guestlist.some" />
        None => println!("You are not on the visitor list. Please leave.")// <callout id="co.guestlist.none" />
    }
    //END: iter_match
}
