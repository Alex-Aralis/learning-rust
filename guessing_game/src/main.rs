extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    
    let secret = rand::thread_rng().gen_range(1,101);

    println!("Make a guess! {}", secret);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Your guessed {}!", guess);
}
