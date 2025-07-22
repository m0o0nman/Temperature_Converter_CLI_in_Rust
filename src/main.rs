use std::io;
//
fn main() {
    println!("==========  Welcome to Temperature Converter!  ==========");
    println!("Select features:\n\n1. Celcius to Farenheit\n2. Farenheit to Celcius");

    let mut input_choice: String = String::new();
    let mut choice: i32 = 0;

    let mut flag: bool = true;

    while flag == true {
        
        io::stdin()
            .read_line(&mut input_choice)
            .expect("Couldnt Read. Try again!");

        let choice: i32 = input_choice
            .trim()
            .parse()
            .expect("Please enter a valid number");

        if choice == 1 || choice == 2 {flag = false;} else {continue;}  
        println!("Wrong choice, try again.") 
    }

    

    print!("Your choice was {}", choice);
    

}
