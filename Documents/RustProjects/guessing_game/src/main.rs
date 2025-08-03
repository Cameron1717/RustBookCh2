use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("enter a number");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("secret: {secret}");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read Line");
    let guess: u32 = guess.trim().parse().expect("please Type a Number");

    match guess.cmp(&secret) {
        Ordering::Less => println!("Higher"),
        Ordering::Greater => println!("Lower"),
        Ordering::Equal => println!("Win!"),
    }
    println!("you entered:{guess}")
}
