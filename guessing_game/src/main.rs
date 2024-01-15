use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guesss the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret Number is {secret_number}");

    'guessing: loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Enter a Number, So ");
                continue;
            }
        };

        println!("Your {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("Congrats, Found the Number");
                break 'guessing;
            }
        }
    }
}
