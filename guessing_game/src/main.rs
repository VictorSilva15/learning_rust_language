use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("\n--- Guess The Number ---\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\nPlease, input your guess:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("\nYou guessed: {}", guess);

        if guess < 1 || guess > 100 {
            println!("\nThe secret number will be between 1 and 100.\n");
            continue;
        }
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
   
}
