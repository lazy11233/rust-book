use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let mut guess_count = 1;
    loop {
        println!("Please input your gusee.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to reade line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                guess_count += 1;
                num
            },
            Err(_) => continue,
        };
    
        println!("You guessed:{guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Guessed count: {guess_count}");
                break;
            }
        }
    }
}
