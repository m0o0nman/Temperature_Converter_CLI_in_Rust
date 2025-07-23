use std::io;

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

    println!("You chose: {}", choice);
    

}
