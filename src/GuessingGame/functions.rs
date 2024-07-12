use std::io::stdin;
use rand::Rng;

pub(crate) struct GuessingGame {
    rand_num: u32,
}

impl GuessingGame {
    pub(crate) fn new() -> GuessingGame {
        let rand_num = rand::thread_rng().gen_range(0..100);

        GuessingGame { rand_num }
    }

    pub(crate) fn get_user_input_num() -> u32 {
        println!("Guess the number!");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let num: u32 = input.trim().parse().expect("Not a valid number");

        return num;
    }
    pub(crate) fn game_logic(&self) {
        loop {
            let user_input = Self::get_user_input_num();

            if user_input > self.rand_num {
                println!("Your guess is too high!");
                continue;
            }
            else if user_input < self.rand_num {
                println!("Your guess is too low");
                continue;
            }
            else if user_input == self.rand_num {
                println!("Congrats!");
                break;
            }
        }
    }

}