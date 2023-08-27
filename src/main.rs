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
    // control flow of the conversion
    let mut conversion_method:&str;
    if conversion_method_input =="1" {
        conversion_method = "Fahrenheit to Celsius";
    } else if conversion_method_input == "2" {
        conversion_method = "Celsius to Fahrenheit";
    } else {
        conversion_method = "else";
    }
    println!("You choose convert method {}-{}", conversion_method, conversion_method_input)
}
