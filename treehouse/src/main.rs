#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let allowed_visitors = [
        Visitor::new("anna", "You're the best!"),
        Visitor::new("bob", "Missing Alice?"),
        Visitor::new("charlie", "Don't be grumpy!"),
        Visitor::new("david", "What's up David!"),
        Visitor::new("emma", "Where's your mom?"),
    ];
    println!("Hello, what's your name?");
    let name = get_name();
    match allowed_visitors.iter().find(|visitor| visitor.name == name) {
        Some(visitor) => visitor.greet(),
        None => println!("You're not on the list. Please leave!"),
    }
}
