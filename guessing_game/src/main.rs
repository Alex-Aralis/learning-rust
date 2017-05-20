extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret = rand::thread_rng().gen_range(1,101);

    println!("Make a guess! {}", secret);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("Your guessed {}!", guess.trim());

        let guess: u32 = guess.trim().parse().expect("PLS GIVE NUMBER.");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => println!("You got it!"),
        }
    }
}
