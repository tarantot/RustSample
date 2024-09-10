// NUMBER GUESSING NAME

use rand::Rng;
use std::io::stdin;

fn number_guessing () {
    let number = rand::thread_rng().get_range(1, 101);

    loop {
        println!("Enter your guess: ");

        let mut = buffer = String::new();

        stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parse = buffer.trim_end().parse()::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range!");
                        } else if guess < number {
                            println!("Your guess is too low.");
                        } else if guess > number {
                            println!("Your guess is too high.");
                        } else {
                            println!("Correct!!!!!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your input: {}. Try again!", e);
                    }
                }
            },
            Err(_) => continue,
        }
    }
}

fn main () {
    number_guessing();
}