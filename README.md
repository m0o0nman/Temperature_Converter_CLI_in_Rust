# Temperature Converter (Rust CLI)

A simple command-line application for converting temperatures between Celsius and Fahrenheit.

## What I Learned

This project helped me practice and understand:

- **Rust basics:** variables, types, and functions
- **Modules:** organizing code with `mod`
- **User input:** reading and parsing input from the terminal
- **Error handling:** using `match` for input validation
- **Control flow:** loops and conditional statements (`if`, `else`)
- **Formatting output:** printing floating-point numbers with two decimals
- **Separation of concerns:** keeping conversion logic and input handling in separate modules

## How to Run

1. **Clone the repository:**
   ```sh
   git clone https://github.com/yourusername/temperature_converter.git
   cd temperature_converter
   ```

2. **Run the program:**
   ```sh
   cargo run
   ```

## Example Output

```
==========  Welcome to Temperature Converter!  ==========
Select features:

1. Celcius to Farenheit
2. Farenheit to Celcius
1

Temp in Celsius
25
25.00C in farenheit: 77.00F.
```

## Project Structure

- `src/main.rs` - Main logic and user interaction
- `src/converter.rs` - Temperature conversion functions
- `src/input.rs` - Input handling

##
