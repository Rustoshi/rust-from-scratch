use std::{io, cmp::Ordering};
use rand::{self, Rng};

fn main(){
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut reward = 1_000_000.00;
    let loss = reward * 0.05;

    println!("Guess the number to win ${reward}");
    println!("You lose 5% of the reward each time you guess wrong");

    loop {
        let mut guess = String::new();
        println!("Guess the number: ");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    
        println!("You guessed {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number between 0 and 100");
                continue
            } 
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won ${reward}, congratulations!");
                break;
            },
            Ordering::Greater => {
                reward -= loss;
                println!("You guessed too high");
                println!("Reward reduced to ${reward}");
            },
            Ordering::Less => {
                reward -= loss;
                println!("You guessed too low");
                println!("Reward reduced to ${reward}");
            }
        }
    }

}