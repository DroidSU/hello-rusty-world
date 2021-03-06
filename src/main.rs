use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess: ");

    let mut guess = String::new();

    let _secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number : {}", _secret_number);

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
