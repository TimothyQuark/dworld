use bevy::prelude::*;

use super::components::geometry::*;
use super::components::map::TileType;
use super::map::Map;
use super::AppState;

mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;

mod common;
use common::apply_room_to_map;

pub fn random_builder_system(mut commands: Commands, mut state: ResMut<State<AppState>>) {
    println!("Generating a new map for depth -1 (TODO)");
    let new_depth = 0;

    let rng = 1;
    let mut result: Box<dyn MapBuilder>;
    match rng {
        1 => {
            result = Box::new(BspDungeonBuilder::new(new_depth));
            result.build_map();
        }
        _ => {
            panic!("Undefined map builder selected");
        }
    }

    // TODO: Get starting position, and also spawn entities
    let new_map = result.get_map();

    // This will rewrite the previous map resource
    commands.insert_resource(new_map);
    state.set(AppState::InGame).unwrap();
    println!("New map created and inserted as a resource");
}

// pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
//     // let mut rng = rltk::RandomNumberGenerator::new();
//     let builder = 1;
//     match builder {
//         1 => Box::new(BspDungeonBuilder::new(new_depth)),
//     }
// }

pub trait MapBuilder {
    fn build_map(&mut self);
    // fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    // fn get_snapshot_history(&self) -> Vec<Map>;
    // fn take_snapshot(&mut self);
}
