use std::io;
//
fn main() {
    println!("==========  Welcome to Temperature Converter!  ==========");
    println!("Select features:\n\n1. Celcius to Farenheit\n2. Farenheit to Celcius");

    let mut input_choice: String = String::new();
    let mut choice: i32 = 0;

    let mut flag: bool = true;

    while flag == true {
        input_choice.clear();
        
        io::stdin()
            .read_line(&mut input_choice)
            .expect("Couldnt Read. Try again!");

        match input_choice
            .trim()
            .parse(){
                Ok(num) if num == 1 || num == 2 => {
                    choice = num;
                    flag = false;
                }
                _ => {
                    print!("Wrong choice, try again!")
                }
            } 
    }

        // let choice: i32 = input_choice
        //     .trim()
        //     .parse()
        //     .expect("Please enter a valid number");

        // if choice == 1 || choice == 2 {flag = false;} else {continue;}  
    

    print!("Your choice was {}", choice);
    

}
