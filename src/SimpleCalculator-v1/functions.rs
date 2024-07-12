use std::io::stdin;
use std::process::exit;

pub(crate) struct Calculator {
    a: f32,
    b: f32,
}

impl Calculator {
    pub(crate) fn new() -> Calculator {
        let a = Calculator::read_number();
        let b = Calculator::read_number();
        Calculator {a,b}
    }

    pub(crate) fn read_number() -> f32 {
        let mut input = String::new();
        println!("Please enter an integer or a floating-point number.");
        stdin().read_line(&mut input).expect("Failed to read line");
        let num: f32 = input.trim().parse().expect("Not a valid number");
        return num;
    }

    pub(crate) fn menu(&self) {
        loop {
            println!("Choose the operation:\n
                1 - Addition\n
                2 - Subtraction\n
                3 - Multiplication\n
                4 - Division\n
                5 - Quit"
            );
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");

            let operation: Result<i32, _> = input.trim().parse();
            match operation {
                Ok(num) if num >= 1 && num <= 5 => {
                    match num {
                        1 => {
                            println!("Result of addition: {}", self.a + self.b)
                        }
                        2 => {
                            println!("Result of subtraction: {}", self.a - self.b)
                        }
                        3 => {
                            println!("Result of multiplication: {}", self.a * self.b)
                        }
                        4 => {
                            if(self.b != 0.0) {
                                println!("Result of division: {}", self.a / self.b)
                            } else { println!("Cannot divide by 0"); }
                        }
                        5 => { exit(1); }
                        _ => { println!("Invalid input. Please enter a number between 1 and 5."); }
                    }

                    break;
                }
                Ok(_) => println!("Invalid input. Please enter a number between 1 and 5."),
                Err(_) => println!("Invalid input. Please enter a valid number."),
            }
        }
    }
}