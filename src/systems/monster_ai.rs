use bevy::prelude::*;

use crate::components::{living::Monster, map::Position};
use crate::systems::map::Map;
use crate::AppState;

pub fn monster_ai(
    map: Res<Map>,
    mut state: ResMut<State<AppState>>,
    query: Query<&Position, With<Monster>>,
) {
    println!("Monster turn");
    state.set(AppState::AwaitingInput).unwrap();
}
