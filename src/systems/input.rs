use bevy::prelude::*;

use crate::components::{living::Player, map::Position};
use crate::systems::map::Map;

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut query: Query<&mut Position, With<Player>>,
) {
    // See https://bevy-cheatbook.github.io/input/keyboard.html

    // TODO: Match statement using current game state, and then use either
    // single key events, or text input for entire sentences (see https://bevy-cheatbook.github.io/input/char.html)

    // let player_pos = query.single_mut();

    // Check diagonal movements before normal movements. Alternatively, check if
    // Shift or control are NOT pressed.
    if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LShift) {
        // println!("Right Shift pressed");
        try_move_player(1, 1, &map, query.single_mut().as_mut());
    } else if keys.just_pressed(KeyCode::Right) && keys.pressed(KeyCode::LControl) {
        // println!("Right Control pressed");
        try_move_player(1, -1, &map, query.single_mut().as_mut());
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LShift) {
        // println!("Right key pressed");
        try_move_player(-1, 1, &map, query.single_mut().as_mut());
    } else if keys.just_pressed(KeyCode::Left) && keys.pressed(KeyCode::LControl) {
        // println!("Right key pressed");
        try_move_player(-1, -1, &map, query.single_mut().as_mut());
    } else if keys.just_pressed(KeyCode::Down) {
        // println!("Down key pressed");
        try_move_player(0, -1, &map, query.single_mut().as_mut());
    } else if keys.just_pressed(KeyCode::Up) {
        // println!("Up key pressed");
        try_move_player(0, 1, &map, query.single_mut().as_mut());
    } else if keys.just_pressed(KeyCode::Left) {
        // println!("Left key pressed");
        try_move_player(-1, 0, &map, query.single_mut().as_mut());
    } else if keys.just_pressed(KeyCode::Right) {
        // println!("Right key pressed");
        try_move_player(1, 0, &map, query.single_mut().as_mut());
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, map: &Map, player_pos: &mut Position) {
    // println!("Trying to move player");

    if player_pos.x + delta_x < 0
        || player_pos.x + delta_x > map.width as i32 - 1
        || player_pos.y + delta_y < 0
        || player_pos.y + delta_y > map.height as i32 - 1
    {
        return;
    }

    let destination_idx = map.xy_idx(player_pos.x + delta_x, player_pos.y + delta_y);

    // Check if destination is blocked
    if !map.blocked_tiles[destination_idx] {
        player_pos.x = player_pos.x + delta_x;
        player_pos.y = player_pos.y + delta_y;
    }
}
