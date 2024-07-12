use std::io::{BufRead, stdin};
use regex::Regex;

pub(crate) struct Calculator {
    a: f32,
    b: f32,
}

impl Calculator {
    pub fn new(a: f32, b: f32) -> Calculator {
        Calculator { a, b }
    }

    pub(crate) fn read_parse_string(&mut self) {
        println!("Enter Expression");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        let regex = Regex::new(r"(\d+|\+|-|\*|/)").expect("Invalid Regex Pattern");

        let mut tokens: Vec<String> = Vec::new();

        for token in regex.find_iter(&input) {
            tokens.push(token.as_str().to_string());
        }

        let num1: f32 = tokens[0].parse().expect("Failed to parse number");
        let operation: &str = &tokens[1];
        let num2: f32 = tokens[2].parse().expect("Failed to parse number");

        self.a = num1;
        self.b = num2;

        self.operation_result(operation);
    }

    pub(crate) fn operation_result(&self, operation: &str) {
        match operation {
            "+" => {
                println!("Result of addition: {}", self.a + self.b)
            }
            "-" => {
                println!("Result of subtraction: {}", self.a - self.b)
            }
            "*" => {
                println!("Result of multiplication: {}", self.a * self.b)
            }
            "/" => {
                if self.b != 0.0 {
                    println!("Result of division: {}", self.a / self.b)
                } else { println!("Cannot divide by 0"); }
            }
            _ => { println!("Invalid input."); }
        }
    }
}