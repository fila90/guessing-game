extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let rand_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter the nubmer:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You can't enter a string {}", guess);
                continue;
            },
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bravooo!");
                println!("Your number: {}, Random number: {}", guess, rand_num);
                break;
            }
        }
    }
}
