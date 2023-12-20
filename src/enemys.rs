use std::cmp::Ordering;
use::std::io;
pub struct Player {
    pub hp: u8,
    pub dmg: u8,
}

pub struct Enemy {
    pub hp: u8,
    pub dmg: u8,
    pub sec_num: u8,
}

pub fn attack_little(player: &mut Player, enemy: &mut Enemy) {
    println!("Attack with your guess (0-10)");
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).expect("Invalid Input");
    let player_guess: u8 = player_input.trim().parse().expect("You must enter a number");

    if player_guess == enemy.sec_num {
        enemy.hp -= player.dmg;
        println!("You killed that enemy.");
    } else {
        player.hp -= enemy.dmg;
        println!("Your guess was wrong, you're hit {} HP left", player.hp);
    }
}

pub fn attack_boss(player: &mut Player, enemy: &mut Enemy) -> bool {
    println!("Attack with your guess (0-100)");
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).expect("Invalid Input");
    let player_guess: u8 = player_input.trim().parse().expect("You must enter a number");

    match player_guess.cmp(&enemy.sec_num) {
        Ordering::Greater => {
            player.hp -= enemy.dmg;
            println!("You guessed high, your HP: {}", player.hp);
            return false;
        }
        Ordering::Less => {
            player.hp -= enemy.dmg;
            println!("You guessed low, your HP: {}", player.hp);
            return false;
        }
        Ordering::Equal => {
            enemy.hp -= player.dmg;
            println!("Your guess was right, boss HP: {}", enemy.hp);
            return true;
        }
    }
}