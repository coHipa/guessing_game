use random_number::random;
use crate::enemys::{Player, Enemy, attack_little, attack_boss};

mod enemys;
fn main() {
    println!("Guess through hell");

    let mut player = Player {hp: 100, dmg: 10};
    
    while player.hp > 0 {
        let enemy_count = random!(1,5);
        let mut boss_enemy = Enemy {hp: 100, dmg: 20};
        
        for i in 1..=enemy_count {
            let mut little_enemy = Enemy {hp: 10, dmg: 5};
           
            attack_little(&mut player, &mut little_enemy)
        }

        attack_boss(&mut player, &mut boss_enemy)
    }
}