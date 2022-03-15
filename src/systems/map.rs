use bevy::prelude::*;

use crate::text::char_to_cp437;

/// Component that describes the kind of tile on a Map
#[derive(PartialEq, Copy, Clone)]
pub enum MapTileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
}

/// Resource that holds the game map
#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<MapTileType>,
    pub width: u32,
    pub height: u32,
    pub blocked: Vec<bool>,
}

impl Default for Map {
    /// Create a tile map of walls
    fn default() -> Self {
        println!("Default map initialized (still need to add as a resource)");
        let width: u32 = 40;
        let height: u32 = 24;

        // Downstairs tiles so it is obvious this should not be used
        Self {
            tiles: vec![MapTileType::DownStairs; (width * height) as usize],
            width,
            height,
            blocked: vec![true; (width * height) as usize],
        }
    }
}

impl Map {
    /// Create a new map consisting of only Wall tiles
    pub fn new(width: u32, height: u32) -> Map {
        let map = Map {
            tiles: vec![MapTileType::Wall; (width * height) as usize],
            width,
            height,
            blocked: vec![true; (width * height) as usize],
        };
        println!("New Map created (still need to add as a resource)");

        map
    }

    /// Converts XY coordinate to index in tile vec
    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        ((y * self.width) + x) as usize
    }

    /// Converts index in tile vec to XY coordinate
    /// returns (x,y)
    pub fn idx_xy(&self, idx: usize) -> (u32, u32) {
        let x = idx as u32 % self.width;
        let y = (idx as u32 - x) / self.width;
        // let y = idx / self.width;

        (x, y)
    }

    pub fn maptile_to_cp437(tile: MapTileType) -> usize {
        match tile {
            MapTileType::Wall => char_to_cp437('#'),
            MapTileType::Floor => char_to_cp437('.'),
            MapTileType::DownStairs => char_to_cp437('↓'),
            MapTileType::UpStairs => char_to_cp437('↑'),
        }
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == MapTileType::Wall;
        }
    }
}

pub fn init_map(mut commands: Commands) {
    let map = Map::default();

    commands.insert_resource(map);
}
