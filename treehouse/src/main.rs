#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

const ALLOWED_VISITORS: [&str; 5] = ["anna", "bob", "charlie", "david", "emma"];

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn is_allowed(name: &str) -> bool {
    ALLOWED_VISITORS.contains(&name)
}

fn main() {
    println!("Hello, what's your name?");
    let name = get_name();
    if is_allowed(&name) {
        println!("Welcome, {name}!");
    } else {
        println!("Sorry, you are not on the list.");
    }
}
