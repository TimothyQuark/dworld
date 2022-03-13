use bevy::prelude::*;

use crate::components::map::*;

/// Resource that holds the game map
#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<MapTileType>,
    pub width: u32,
    pub height: u32,
}

impl Default for Map {
    /// Create a 200 by 100 tile map of walls
    fn default() -> Self {
        println!("Default map initialized (still need to add as a resource)");
        let width: u32 = 200;
        let height: u32 = 100;

        // Downstairs used so it is obvious this should not be used
        Self {
            tiles: vec![MapTileType::DownStairs; (width * height) as usize],
            width,
            height,
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
    pub fn idx_xy(&self, idx: u32) -> (u32, u32) {
        let x = idx % self.width;
        let y = (idx - x) / self.width;
        // let y = idx / self.width;

        (x, y)
    }

    pub fn maptiletype_to_spriteidx(tile: MapTileType) -> usize {
        match tile {
            MapTileType::Wall => 35,
            MapTileType::Floor => 46,
            MapTileType::DownStairs => 25,
            MapTileType::UpStairs => 24,
        }
    }
}

pub fn init_map(mut commands: Commands) {
    let map = Map::default();

    commands.insert_resource(map);
}

// /// System which transfers map data to the terminal so it can be rendered.
// /// Does not actually render anything itself, important for choosing
// /// which tiles to display
// pub fn render_map(
//     map: Res<Map>,
//     mut terminal: ResMut<Terminal>
// ) {

//     // TODO: Take player position, and render the tiles around them

//     //

//     // for x in 1..render_width {
//     //     for y in 0..render_width {

//     //     }
//     // }

//     // terminal.terminal_tiles[0].0 = 1;
//     // terminal.terminal_tiles[2].0 = 1;
//     // terminal.terminal_tiles[4].0 = 1;

// }
