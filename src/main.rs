use std::io;

fn main() {
    println!("Welcome to Fahrenheit, Celsius Converter....");
    println!("What would you like to convert from and to?");
    println!("1. Fahrenheit to Celsius\n2. Celsius to Fahrenheit");
    // choose conversion method input
    let mut conversion_method = String::new();
    io::stdin()
        .read_line(&mut conversion_method)
        .expect("Failed to read line");
    println!("You choose convert method {}", conversion_method)
}
