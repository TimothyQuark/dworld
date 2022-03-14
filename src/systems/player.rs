use bevy::prelude::*;

use crate::components::{map::Position, player::Player, rendering::Renderable};

/// Spawn the player entity
pub fn init_player(mut commands: Commands) {
    println!("Player initialized");
    // Simple way to create an entity and return an id directly inside the function.
    let _player = commands
        .spawn()
        .insert(Player)
        .insert(Renderable {
            glyph: '@',
            fg: Color::WHITE,
            bg: Color::BLACK,
            render_order: 2,
        })
        .insert(Position { x: 0, y: 0 })
        .id();
}
