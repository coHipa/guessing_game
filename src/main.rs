use random_number::random;
use crate::enemys::{Player, Enemy, attack_little, attack_boss};

mod enemys;
fn main() {
    println!("Guess through hell");
    println!("You wake up in hell and have to guess your way out.");

    let mut player = Player {hp: 100, dmg: 10};
    
    while player.hp > 0 {
        let enemy_count = random!(1,5);
        let mut little_enemy = Enemy {hp: 10, dmg: 5, sec_num: random!(1, 10)};
        let mut boss_enemy = Enemy {hp: 100, dmg: 20, sec_num: random!(1, 100)};
        let mut is_dead = false;
        
        for _i in 1..=enemy_count {   
            attack_little(&mut player, &mut little_enemy)
        }
        while !is_dead{
            is_dead = attack_boss(&mut player, &mut boss_enemy); 
        }
    }
}