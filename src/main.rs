use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1,100);

    println!("Please input your guess.");
    
    loop {
        let mut guess : String = String::new();
              io::stdin()
            .read_line( &mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse(){ // Shadowing of variable type
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a numerical value!");
                continue
            }
        };
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}