use rand::Rng;
use std::io;

fn main() {
    println!("enter a number");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("secret: {secret}");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read Line");

    println!("you entered:{guess}")
}
