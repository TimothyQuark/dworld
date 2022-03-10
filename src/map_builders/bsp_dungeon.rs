use bevy::math::Rect;

use super::{Map, MapBuilder, Position};
use crate::{components::map::TileType, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct BspDungeonBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    rooms: Vec<Rect<i32>>,
    // history: Vec<Map>,
    rects: Vec<Rect<i32>>,
}

impl MapBuilder for BspDungeonBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn build_map(&mut self) {
        self.build();
    }

    // fn spawn_entities(&mut self, ecs : &mut World) {
    //     for room in self.rooms.iter().skip(1) {
    //         spawner::spawn_room(ecs, room, self.depth);
    //     }
    // }
}

impl BspDungeonBuilder {
    pub fn new(new_depth: i32) -> BspDungeonBuilder {
        println!("New BspDungeonBuilder created (map needs to be built)");
        BspDungeonBuilder {
            // TODO: Decouple map size from screen dimensions
            map: Map::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            starting_position: Position { x: 0, y: 0 },
            depth: new_depth,
            rooms: Vec::new(),
            // history: Vec::new(),
            rects: Vec::new(),
        }
    }

    fn build(&mut self) {
        println!("BspDungeonBuilder built");
        for x in 0..self.map.width {
            for y in 0..self.map.height {
                let idx = self.map.xy_idx(x, y) as usize;
                // let mut tile = &map.tiles[idx];
                // let mut tile = map.as_ref().tiles[idx];
                // let tiles = mut map.tiles[idx];
                // *tiles = TileType::Floor;
                self.map.tiles[idx] = TileType::Floor;
            }
        }
        self.map.tiles[15] = TileType::Wall;
    }
}
