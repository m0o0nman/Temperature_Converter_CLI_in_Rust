use std::io;
mod converter;

fn main() {
    println!("==========  Welcome to Temperature Converter!  ==========");
    println!("Select features:\n\n1. Celcius to Farenheit\n2. Farenheit to Celcius");

    let mut input_choice: String = String::new();
    let choice: i32;

    loop {
        input_choice.clear();

        io::stdin()
            .read_line(&mut input_choice)
            .expect("Couldn't read line");

        match input_choice.trim().parse() {
            Ok(num) if num == 1 || num == 2 =>{
                choice = num;
                break;
            },

            Ok(_) => {
                println!("Please enter a number between 1 or 2.")
            }

            Err(_) => {
                println!("Please enter a number.");
            }
        }
    }

    if choice == 1 {
        println!("Enter temp in celcius: ");
        let mut input: String = String::new();
        let output: f64;

        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a number");

        output = input
            .trim()
            .parse()
            .expect("Couldn't read line.");

        println!("{:.2}C in farenheit: {:.2}F.", output, converter::celcius_to_farenheit(output));
    } else {
        println!("Enter temp in farenheit: ");
        let mut input: String = String::new();
        let output: f64;

        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a number");

        output = input
            .trim()
            .parse()
            .expect("Couldn't read line.");

        println!("{:.2}F in celcius: {:.2}C.", output, converter::farenheit_to_celcius(output));
    }
}
