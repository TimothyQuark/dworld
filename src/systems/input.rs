use bevy::prelude::*;

pub fn player_input(keys: Res<Input<KeyCode>>) {
    // See https://bevy-cheatbook.github.io/input/keyboard.html

    // TODO: Match statement using current game state, and then use either
    // single key events, or text input for entire sentences (see https://bevy-cheatbook.github.io/input/char.html)

    if keys.just_pressed(KeyCode::Down) {
        println!("Down key pressed");
    }
    if keys.just_pressed(KeyCode::Up) {
        println!("Up key pressed");
    }
    if keys.just_pressed(KeyCode::Left) {
        println!("Left key pressed");
    }
    if keys.just_pressed(KeyCode::Right) {
        println!("Right key pressed");
    }
}

// fn try_move_player(delta_x, delta_y) {

// }
