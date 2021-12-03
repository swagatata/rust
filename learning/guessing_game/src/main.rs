use std::io;
use rand::Rng;

fn main() {
    println!("What is your name?");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Are you sure you are {}, {}?", guess, secret_number);
}
