use rand::Rng;

use std::cmp::Ordering;
use std::io;

fn main() {
    let n = 100;
    println!("Guess the number between 1 and {n}!");

    let secret_number = rand::thread_rng().gen_range(1..=n);

    println!("Please input your guess.");

    while {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<usize>() {
            Ok(num) => {
                println!("You guessed: {num}");
                match num.cmp(&secret_number) {
                    Ordering::Less => {
                        println!("Too small! Try again!");
                        true
                    }
                    Ordering::Greater => {
                        println!("Too big! Try again!");
                        true
                    }
                    Ordering::Equal => {
                        println!("You win!");
                        false
                    }
                }
            }
            Err(_) => {
                println!("Please type a number!");
                true
            }
        }
    } {} // that's a do-while loop
}
