mod functions;

use std::io::stdin;
use functions::Converter;

fn main() {
    println!(
        "Please enter the type of conversion you'd like to do:
        1 - Celsius to Fahrenheit
        2 - Fahrenheit to Celsius
        3 - Celsius to Kelvin
        4 - Kelvin to Celsius
        5 - Fahrenheit to Kelvin
        6 - Kelvin to Fahrenheit
    ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let input_operation: i32 = input.trim().parse().expect("Please enter a number");

    println!("Please enter the value to convert:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let value: f64 = input.trim().parse().expect("Please enter a valid number");

    match input_operation {
        1 => println!("{}", Converter::celsius_to_fahrenheit(value)),
        2 => println!("{}", Converter::fahrenheit_to_celsius(value)),
        3 => println!("{}", Converter::celsius_to_kelvin(value)),
        4 => println!("{}", Converter::kelvin_to_celsius(value)),
        5 => println!("{}", Converter::fahrenheit_to_kelvin(value)),
        6 => println!("{}", Converter::kelvin_to_fahrenheit(value)),
        _ => {
            println!("Invalid choice");
        }
    }
}

