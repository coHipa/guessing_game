use random_number::random;
use crate::enemys::{Player, Enemy, attack_little, attack_boss};

mod enemys;
fn main() {
    println!("Guess through hell");

    let mut player = Player {hp: 100, dmg: 10};
    let mut little_enemy = Enemy {hp: 10, dmg: 5};
    let mut boss_enemy = Enemy {hp: 100, dmg: 20};

    while player.hp > 0 {
        let enemy_count = random!(1,5);

        for i in enemy_count {
            little_enemy.hp.clear();

            attack_little(&mut player, &mut little_enemy)
        }

        attack_boss(&mut player, &mut boss_enemy)
    }
}
