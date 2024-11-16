use crate::models::character::Character;

/// HPを更新する関数
pub fn update_status(player: &mut Character, enemy: &mut Character, previous_player_hp: i32, previous_enemy_hp: i32) {
    // HPの変化をチェック
    if player.stats.hp != previous_player_hp || enemy.stats.hp != previous_enemy_hp {
        // 状態を更新
        display_hp_bar(&player.name, player.stats.hp, player.stats.hp);
        display_hp_bar(&enemy.name, enemy.stats.hp, enemy.stats.hp);
    }

    clear_screen();
}

#[allow(dead_code)]
/// ATBバーを表示する関数
pub fn display_atb_bar(character_name: &str, atb: i32) {
    let atb = atb.min(100);
    let bar_length = 20;
    let filled_length = (atb / 5) as usize;
    let empty_length = bar_length - filled_length;

    let bar = format!(
        "[{}>{}]",
        "█".repeat(filled_length),
        " ".repeat(empty_length)
    );

    println!("{}のATB: {}", character_name, bar);
}

/// HPバーを表示する関数
pub fn display_hp_bar(character_name: &str, hp: i32, max_hp: i32) {
    // 現在のHPを最大HPで正しく表示する
    let hp_percentage = (hp as f32 / max_hp as f32) * 100.0;
    let bar_length = 20;
    let filled_length = (hp_percentage / 5.0) as usize;
    let empty_length = bar_length - filled_length;

    let bar = format!(
        "{} HP: {} [{}{}]",
        character_name,
        hp,
        "█".repeat(filled_length),
        " ".repeat(empty_length)
    );

    println!("{}", bar);
}


/// ターミナル画面をクリアする関数
pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}