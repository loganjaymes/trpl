use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    
    println!("secnum: {secret}");
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // could've been 'io::stdin().read_line(&mut guess).expect("failed...);

        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret){
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("GG");
                break;
            }
        }

    }
}
