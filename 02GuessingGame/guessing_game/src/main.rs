use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");
        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        let guess_number: u32 = match guess_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess_number}");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}