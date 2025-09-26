use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number");

    println!("Please Input Your Guess");
    loop {
        let mut input_num = String::new();

        let secret_number = rand::rng().random_range(1..=101);

        println!("Secret Number {}", secret_number);
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line");

        println!("You guessed: {}", input_num);

        let input_num: u32 = input_num.trim().parse().expect("Failed to parse");
        match input_num.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            _ => println!("{}", "You lose!".red()),
        }
    }
}
