
use std::io; // std: standard library: https://doc.rust-lang.org/std/prelude/index.html
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..=10);
    // println!("{} is the secret number", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess // Shadowing this var, instead of creating new one (guess_str)
            .trim() // trim spaces at the begining and the end of this string and eliminate \n of read_line (user have to press Enter)
            .parse() // convert this var to destination type, we specified u32
            .expect("Please type a number!"); // error handling when Rust can't convert this string

        // error handling
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };
        
        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Ho ho, too small"),
            Ordering::Greater => println!("Ho ho, too big"),
            Ordering::Equal => {println!("Bingo, it's {secret_number}"); break},
        }
    }
}
