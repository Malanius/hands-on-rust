// Turning Clippy up to pedantic can feel like an exercise in masochism,
// but it's a great way to improve your code.
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let my_list = ["One", "Two", "Three"];
    for item in &my_list {
        println!("{item}");
    }
}
