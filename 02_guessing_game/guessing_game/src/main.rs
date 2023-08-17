use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);
    
    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "The number was too small".red()),
            Ordering::Greater => println!("{}", "The number was too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
