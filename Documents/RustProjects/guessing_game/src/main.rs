use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("enter a number");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("secret: {secret}");
    loop {
        println!("enter guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Higher"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => {
                println!("Win!");
                break;
            }
        }
        println!("you entered:{guess}")
    }
}
