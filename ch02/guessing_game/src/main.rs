use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);
    
    println!("Please input your guess.");

    let mut guess = String::new(); // :: indicates new() is an associated function, which is implemented on a type, not an instance.

    io::stdin().read_line(&mut guess) 
        .expect("Failed to read line"); // if you don't call .expect() here, Rust will compile, but raise a warning "Unused std::result::Result which must be used"

    println!("You guessed: {}", guess);
}

