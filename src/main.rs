use logic::start_battle::start_battle;
use models::character::Character;

mod logic;
mod models;
mod utils;

fn main() {
    let player = Character::new("バッツ", 5420, 3550, 560, 5);
    let enemy = Character::new("ギルガメッシュ", 50000, 2410, 700, 3);

    start_battle(player, enemy);
}
