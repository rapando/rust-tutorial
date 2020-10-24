use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("\n\nGuess the number!");

    loop {
        println!("\n=============================================");
        let secret_number = rand::thread_rng().gen_range(1, 4);
        println!("Enter your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert guess from string to unsigned int32
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input. Ignoring...");
                continue;
            }
        };

        println!("You guessed {}", guess);
        println!("The secret number is {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(">> Too Small"),
            Ordering::Greater => println!(">> Too Big"),
            Ordering::Equal => {
                println!(">> You win!");
                break;
            }
        }
    }
}
