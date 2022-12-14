use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("This is a random number: {}", secret_number);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed");
        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("{}", "Too Small...".red()),
            Ordering::Greater => print!("{}", "Too Big...".red()),
            Ordering::Equal => {
                print!("{}", "You win...".green());
                break;
            }
        }
    }
}
