#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
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
    let mut visitors = vec![
        Visitor::new("anna", "You're the best!"),
        Visitor::new("bob", "Missing Alice?"),
        Visitor::new("charlie", "Don't be grumpy!"),
        Visitor::new("david", "What's up David!"),
        Visitor::new("emma", "Where's your mom?"),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = get_name();
        if let Some(visitor) = visitors.iter().find(|visitor| visitor.name == name) {
            visitor.greet();
            continue;
        }

        if name.is_empty() {
            break;
        }

        println!("{name} is not on the guest list.");
        visitors.push(Visitor::new(&name, "New friend!"));
    }

    println!("The final list of visitors:");
    println!("{visitors:#?}");
}
