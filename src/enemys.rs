use std::cmp::Ordering;
use::std::io;
use random_number::random;
pub struct Player {
    pub hp: u8,
    pub dmg: u8,
}

pub struct Enemy {
    pub hp: u8,
    pub dmg: u8,
}

pub fn attack_little(player: &mut Player, enemy: &mut Enemy) {
    let mut player_input = String::new();
    let secret_number = random!(1,10);

    println!("Attack this little guy (1-10)");
    io::stdin().read_line(&mut player_input).expect("Invalid Input");
    let player_guess: u8 = player_input.trim().parse().expect("You must enter a number");

    if player_guess == secret_number {
        enemy.hp -= player.dmg;
        println!("You killed that enemy.");
    } else {
        player.hp -= enemy.dmg;
        println!("Your guess was wrong, you're hit {} HP left", player.hp);
    }
}

pub fn attack_boss(player: &mut Player, enemy: &mut Enemy) {
    let mut player_input = String::new();
    let mut secret_number = random!(1,50);
    println!("Fight that boss (1-50)");
    
    while enemy.hp > 0 && player.hp > 0 {
        player_input.clear();
        io::stdin().read_line(&mut player_input).expect("Invalid Input");
        let player_guess: u8 = player_input.trim().parse().expect("You must enter a number");

        match player_guess.cmp(&secret_number) {
            Ordering::Greater => {
                player.hp -= enemy.dmg;
                println!("You guessed high, your HP: {}", player.hp);
            }
            Ordering::Less => {
                player.hp -= enemy.dmg;
                println!("You guessed low, your HP: {}", player.hp);
            }
            Ordering::Equal => {
                enemy.hp -= player.dmg;
                println!("Your guess was right, boss HP: {}", enemy.hp);
                secret_number = random!(1,50);
            }
        }
    }
}