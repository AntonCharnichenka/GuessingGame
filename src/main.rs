use rand::Rng; // Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods
use std::cmp::Ordering;
use std::io; // :: means using associated function which is not a method in Rust

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // range expression as an argument

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "exit" {
            println!("Exiting the game");
            break;
        }

        println!("Your guess is {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            } // The underscore, _, is a catchall value
        }; // Shadowing values: when you want to convert a value from one type to another type.

        match guess.cmp(&secret_number) {
            // The match expression ends after the first successful match
            Ordering::Less => println!("Too small!"), // arm of the match expression
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
