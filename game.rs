use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io; // ✅ correct crate name

fn main() {
    println!("Enter a number:");

    // secret_number must match guess type (u32)
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
