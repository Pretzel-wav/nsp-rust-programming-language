use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101); // inclusive on lower bound, exclusive on upper bound

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // :: indicates new() is an associated function, which is implemented on a type, not an instance.

        io::stdin().read_line(&mut guess) 
            .expect("Failed to read line"); // if you don't call .expect() here, Rust will compile, but raise a warning "Unused std::result::Result which must be used"

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

