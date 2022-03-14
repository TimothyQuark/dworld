use bevy::prelude::*;

use crate::components::map::{MapTileType, Position};
use super::{
    common::{apply_room_to_map, draw_corridor},
    Map, MapBuilder,
};
use crate::geometry::Rect;

pub struct EmptyRoomBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    rooms: Vec<Rect>,
    // history: Vec<Map>,
    rects: Vec<Rect>,
}

impl MapBuilder for EmptyRoomBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn build_map(&mut self) {
        self.build();
    }

    fn spawn_entities(&mut self, ecs: &mut World) {}

    // fn spawn_entities(&mut self, ecs : &mut World) {
    //     for room in self.rooms.iter().skip(1) {
    //         spawner::spawn_room(ecs, room, self.depth);
    //     }
    // }
}

impl EmptyRoomBuilder { 

    pub fn new(new_depth: i32) -> Self {
        println!("New EmptyRoomBuilder created (map needs to be built)");
        EmptyRoomBuilder {
            // TODO: Decouple map size from screen dimensions
            map: Map::new(40, 24),
            starting_position: Position { x: 0, y: 0 },
            depth: new_depth,
            rooms: Vec::new(),
            // history: Vec::new(),
            rects: Vec::new(),
        }
    }

    fn build(&mut self) {

        self.rects.clear();
        self.rects.push(Rect::new(
            1,
            1,
            self.map.width as i32 - 4,
            self.map.height as i32 - 4,
        )); // Start with a single map-sized rectangle
        let first_room: Rect = self.rects[0];

        apply_room_to_map(&mut self.map, &first_room);
        // println!(
        //     // "x1: {}, x2: {}, y1: {}, y2: {}",
        //     first_room.x1, first_room.x2, first_room.y1, first_room.y2
        // );
    }
}
