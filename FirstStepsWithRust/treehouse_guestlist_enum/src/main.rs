use std::io::stdin;

// START: enum
#[derive(Debug)]    // <callout id="co.derive" />
enum VisitorAction {  // <callout id="co.enum" />
  Accept,         // <callout id="co.accept" />
  AcceptWithNote { note: String },    // <callout id="co.note" />
  Refuse,
  Probation,
} // <callout id="co.nosemicolon" />
// END: enum


// START: guest
#[derive(Debug)]
struct Visitor {
  name: String,
  action: VisitorAction,    // <callout id="co.action" />
  age: i8                // <callout id="co.int" />
}
// END: guest

impl Visitor {
  // START: constructor
  fn new(name: &str, action: VisitorAction, age: i8) -> Self { // <callout id="co.vis.constructor" />
    Self {
      name: name.to_lowercase(), // <callout id="co.vis.name" />
      action, // <callout id="co.vis.action" />
      age
    }
  }
  // END: constructor
  // START: greeter
  fn greet_visitor(&self) {
    match &self.action {
      VisitorAction::Accept => println!("Welcome to the tree 
          house, {}", self.name), // <callout id="co.match.accept" />
      VisitorAction::AcceptWithNote { note } => { // <callout id="co.match.accept_note" />
          println!("Welcome to the treehouse, {}", self.name);
          println!("{}", note); // <callout id="co.match.accept_note_text" />
          if self.age < 21 { // <callout id="co.match.intmath" />
              println!("Do not serve alcohol to {}", self.name);
          }
      }
      VisitorAction::Probation => println!("{} is now a 
          probationary member", self.name),
      VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
      }
  }
  // END: greeter
}

fn what_is_your_name() -> String {
  let mut your_name = String::new();
  stdin()
      .read_line(&mut your_name)
      .expect("Failed to read line");
  your_name.trim().to_lowercase()
}

//START: visitor_init
fn main() {
  let mut visitor_list = vec![
    Visitor::new("Bert", VisitorAction::Accept, 45),
    Visitor::new("Steve", VisitorAction::AcceptWithNote{ 
        note: String::from("Lactose-free milk is in the fridge") 
    }, 15),
    Visitor::new("Fred", VisitorAction::Refuse, 30),
  ];
  //END: visitor_init
  loop {
    println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
    let name = what_is_your_name();
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
      Some(visitor) => visitor.greet_visitor(),
      None => {
        if name.is_empty() {
          break;
        } else {
          println!("{} is not on the visitor list.", name);
          visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
      }
    }
  }
  println!("The final list of visitors:");
  println!("{:#?}", visitor_list);
}
