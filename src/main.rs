use std::{io, cmp::Ordering};
use random_number::random;
fn main() {
    println!("Guess through hell");
    
    let secret_number: u8 = random!(0, 100);
    let mut player_health = 100;

    loop {        
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Invalid input");

        let guess: u8 = match guess.trim().parse() {
            Ok(random_number) => random_number,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => {
                player_health -= 10;
                println!("You gussed to high, remaining health: {}", player_health);
            }
            Ordering::Less => {
                player_health -= 10;
                println!("Your guess was to low, remaining health: {}", player_health);
            }
  
        };            

        if player_health < 1 {
            println!("You lost all your HP, start a new game.");
            break;
        }
    }
}
