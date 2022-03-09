use bevy::prelude::*;

use super::map::Map;
use super::{AppState, TileType};

pub fn new_map_system(mut map: ResMut<Map>, mut state: ResMut<State<AppState>>) {
    println!("Generating a new map");

    for x in 0..map.width {
        for y in 0..map.height {
            let idx = map.xy_idx(x, y) as usize;
            // let mut tile = &map.tiles[idx];
            // let mut tile = map.as_ref().tiles[idx];
            // let tiles = mut map.tiles[idx];
            // *tiles = TileType::Floor;
            map.tiles[idx] = TileType::Floor;
        }
    }

    state.set(AppState::InGame).unwrap();
}

// pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
//     // let mut rng = rltk::RandomNumberGenerator::new();
//     let builder = 1;
//     match builder {
//         1 => Box::new(BspDungeonBuilder::new(new_depth)),
//     }
// }

// pub trait MapBuilder {
//     fn build_map(&mut self);
//     fn spawn_entities(&mut self, ecs: &mut World);
//     fn get_map(&self) -> Map;
//     fn get_starting_position(&self) -> Position;
//     fn get_snapshot_history(&self) -> Vec<Map>;
//     fn take_snapshot(&mut self);
// }
