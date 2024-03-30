#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}
#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}!", self.name);
                println!("{note}");
                if self.age < 21 {
                    println!("Do not serve alcohol to {}.", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member.", self.name),
            VisitorAction::Refuse => println!("Sorry, {}. You're not welcome here.", self.name),
        }
    }
}

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let mut visitors = vec![
        Visitor::new(
            "anna",
            VisitorAction::AcceptWithNote {
                note: String::from("You're the best!"),
            },
            20,
        ),
        Visitor::new("bob", VisitorAction::Accept, 33),
        Visitor::new("charlie", VisitorAction::Refuse, 44),
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
        visitors.push(Visitor::new(&name, VisitorAction::Probation, 0));
    }

    println!("The final list of visitors:");
    println!("{visitors:#?}");
}
