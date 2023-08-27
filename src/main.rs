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
    // enter conversion value
    println!("Enter the conversion value: ");
    let conversion_method_input: u32 = match conversion_method_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please enter a number");
        }
    };
    let mut conversion_value_input = String::new();
    io::stdin()
        .read_line(&mut conversion_value_input)
        .expect("Failed to read line");
    let _conversion_value_input: f32 = match conversion_value_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please enter a number");
        }
    };
    // control flow of the conversion
    if conversion_method_input == 1 {
        println!("You choose convert Fahrenheit to Celsius method")
    }
    else if conversion_method_input == 1 {
        println!("You choose convert Celsius to Fahrenheit method")
    }
    else {
        panic!("That conversion is not listed here.")
    }
}
fn fahrenheit_to_celsius(value: f32) -> f32 {
    (value - 32) * 5 / 9
}
fn celsius_to_fahrenheit(value: f32) -> f32 {
    ((value * 9) / 5) + 32
}