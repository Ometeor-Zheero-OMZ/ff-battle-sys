use std::thread;
use std::time::{Duration, Instant};

use crate::models::character::Character;
use crate::utils::update_screen::{clear_screen, display_hp_bar, update_status};

pub fn start_battle(mut player: Character, mut enemy: Character) {
    println!("戦闘開始！\n");

    let mut player_last_atb_update = Instant::now();
    let mut enemy_last_atb_update = Instant::now();

    // 前回の状態を追跡
    let previous_player_hp = player.stats.hp;
    let previous_enemy_hp = enemy.stats.hp;

    // 戦闘開始時のステータスを表示
    // clear_screen();
    display_hp_bar(&player.name, player.stats.hp, player.stats.hp);
    display_hp_bar(&enemy.name, enemy.stats.hp, enemy.stats.hp);
    print!("\n");

    loop {
        // 現在の時間を取得
        let now = Instant::now();

        // プレイヤーのATBゲージ更新
        if now.duration_since(player_last_atb_update).as_millis() >= player.stats.speed as u128 {
            player.fill_atb();
            player_last_atb_update = now;
        }

        // 敵のATBゲージ更新
        if now.duration_since(enemy_last_atb_update).as_millis() >= enemy.stats.speed as u128 {
            enemy.fill_atb();
            enemy_last_atb_update = now;
        }

        // 初期テキストをクリア
        clear_screen();

        // プレイヤーの行動
        if player.is_ready() {
            display_hp_bar(&player.name, player.stats.hp, player.stats.hp);
            display_hp_bar(&enemy.name, enemy.stats.hp, enemy.stats.hp);
            print!("\n");

            let action = get_player_action();

            match action.as_str() {
                "attack" => {
                    player.attack_target(&mut enemy);
                }
                "defend" => {
                    player.defend();
                }
                "skill" => {
                    player.skill_attack(&mut enemy);
                }
                "flee" => {
                    println!("{}はにげだした。", player.name);
                    break;
                }
                _ => println!("無効な選択です。"),
            }

            // ステータスを更新
            update_status(&mut player, &mut enemy, previous_player_hp, previous_enemy_hp);

            if !enemy.is_alive() {
                // clear_screen();
                println!("{}をたおした！", enemy.name);
                break;
            }
        }

        // 敵の行動
        if enemy.is_ready() {
            enemy.attack_target(&mut player);

            // ステータスを更新
            update_status(&mut player, &mut enemy, previous_player_hp, previous_enemy_hp);

            if !player.is_alive() {
                // clear_screen();
                println!("{}は{}とのしょうぶにやぶれた...", player.name, enemy.name);
                break;
            }
        }

        // 遅延
        thread::sleep(Duration::from_millis(100));
    }
}

fn get_player_action() -> String {
    println!("攻撃(attack) | 防御(defend) | クリスタルの導き(skill) | にげる(flee)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    clear_screen();

    input.trim().to_lowercase()
}