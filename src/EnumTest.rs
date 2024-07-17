use std::io::stdin;

#[derive(Debug)]
enum DaysOfTheWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}
fn main() {
    println!("Provide a number to match to day of the week.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let num: u32 = input.trim().parse().expect("Not a valid number");

    match num {
        1 => { println!("{:?}", DaysOfTheWeek::Monday) }
        2 => { println!("{:?}", DaysOfTheWeek::Tuesday) }
        3 => { println!("{:?}", DaysOfTheWeek::Wednesday) }
        4 => { println!("{:?}", DaysOfTheWeek::Thursday) }
        5 => { println!("{:?}", DaysOfTheWeek::Friday) }
        6 => { println!("{:?}", DaysOfTheWeek::Saturday) }
        7 => { println!("{:?}", DaysOfTheWeek::Sunday) }
        _ => { println!("Invalid Number") }
    }
}