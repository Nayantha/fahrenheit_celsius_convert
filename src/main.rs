use std::io;

fn main() {
    println!("Welcome to Fahrenheit, Celsius Converter....");
    println!("What would you like to convert from and to?");
    println!("1. Fahrenheit to Celsius\n2. Celsius to Fahrenheit");
    // choose conversion method input
    let mut conversion_method_input = String::new();
    io::stdin()
        .read_line(&mut conversion_method_input)
        .expect("Failed to read line");
    let conversion_method_input: u32 = match conversion_method_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            0
        }
    };
    // control flow of the conversion
    let conversion_method:&str;
    if conversion_method_input == 1 {
        conversion_method = "Fahrenheit to Celsius";
    } else if conversion_method_input == 1 {
        conversion_method = "Celsius to Fahrenheit";
    } else {
        panic!("That conversion is not listed here.")
    }
    println!("You choose convert method {}", conversion_method)
}
