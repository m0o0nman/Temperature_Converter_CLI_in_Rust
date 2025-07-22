use std::io;
//
fn main() {
    let mut input: String = String::new();

    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read your name.");
    let name = input.trim();

    let mut input_age: String = String::new();
    println!("Enter your age: ");
    io::stdin()
        .read_line(&mut input_age)
        .expect("Failed to read your age.");
    let age: i32 = input_age
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("Your name: {}\nyour age: {}", name, age);

}
