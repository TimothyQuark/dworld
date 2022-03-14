use bevy::prelude::*;

use crate::AppState;

use crate::systems::map::Map;

use crate::components::map::Position;

mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;

mod empty_room;
use empty_room::EmptyRoomBuilder;

mod common;
// use common::apply_room_to_map;

pub fn build_new_map(mut commands: Commands, mut state: ResMut<State<AppState>>) {
    let new_depth = 1;

    // let mut rng_gen = SmallRng::seed_from_u64(100);
    // let rng = rng_gen.gen_range(0..1));
    let rng = 2;
    let mut result: Box<dyn MapBuilder>;

    match rng {
        1 => {
            result = Box::new(BspDungeonBuilder::new(new_depth));
            result.build_map();
        }
        2 => {
            result = Box::new(EmptyRoomBuilder::new(new_depth));
            result.build_map();
        }
        

        _ => {
            panic!("Undefined map builder selected: {}", rng);
        }
    }

    let new_map = result.get_map();

    // This will rewrite the previous map resource
    commands.insert_resource(new_map);
    state.set(AppState::InGame).unwrap();
    println!("New map created and inserted as a resource");
}

// pub fn random_builder_system(mut commands: Commands, mut state: ResMut<State<AppState>>) {
//     println!("Generating a new map for depth -1 (TODO)");
//     let new_depth = 0;

//     let rng = 1;
//     let mut result: Box<dyn MapBuilder>;
//     match rng {
//         1 => {
//             result = Box::new(BspDungeonBuilder::new(new_depth));
//             result.build_map();
//         }
//         _ => {
//             panic!("Undefined map builder selected");
//         }
//     }

//     // TODO: Get starting position, and also spawn entities
//     let new_map = result.get_map();

//     // This will rewrite the previous map resource
//     commands.insert_resource(new_map);
//     state.set(AppState::InGame).unwrap();
//     println!("New map created and inserted as a resource");
// }

// pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
//     // let mut rng = rltk::RandomNumberGenerator::new();
//     let builder = 1;
//     match builder {
//         1 => Box::new(BspDungeonBuilder::new(new_depth)),
//     }
// }

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    // fn get_snapshot_history(&self) -> Vec<Map>;
    // fn take_snapshot(&mut self);
}
