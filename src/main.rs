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

    if choice == 1 {
        println!("Enter temp in celcius: ");
        let mut input: String = String::new();
        let output: f32;

        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a number");

        output = input
            .trim()
            .parse()
            .expect("Couldn't read line.");

        println!("{:.2}C in farenheit: {:.2}F.", output, c_to_f(output));
    } else {
        println!("Enter temp in farenheit: ");
        let mut input: String = String::new();
        let output: f32;

        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a number");

        output = input
            .trim()
            .parse()
            .expect("Couldn't read line.");

        println!("{:.2}F in celcius: {:.2}C.", output, f_to_c(output));
    }
    

}

fn c_to_f(celcius: f32) -> f32 {
    (1.8 * celcius) + 32.0
}

fn f_to_c (farenheit: f32) -> f32 {
    (farenheit - 32.0) * 0.56
}