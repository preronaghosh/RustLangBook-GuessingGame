use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    //println!("Secret num is: {}", secret_num);
    loop {
        println!("Guess a number and give your input..");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guess is: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Don't enter anything other than a number!");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too high".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
