use bevy::prelude::*;

use super::{screentiles_iterator, TerminalTile, SCREEN_HEIGHT, SCREEN_WIDTH, TILESIZE};

// Map width and height, in number of tiles
pub const MAPWIDTH: usize = (SCREEN_WIDTH / TILESIZE) as usize;
pub const MAPHEIGHT: usize = (SCREEN_HEIGHT / TILESIZE) as usize;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
}

pub struct Map {
    /// (0,0) is the bottom left most tile
    pub tiles: Vec<TileType>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    /// Converts XY coordinate to index in tile vec
    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Converts index in tile vec to XY coordinate
    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        let x: usize = idx % self.width;
        let y = (idx - x) / self.height;

        (x, y)
    }

    pub fn new() -> Map {
        let map = Map {
            tiles: vec![TileType::Wall; MAPCOUNT],
            width: MAPWIDTH,
            height: MAPHEIGHT,
        };

        // Used for testing
        // map.tiles[0] = TileType::Floor;
        // map.tiles[3] = TileType::Floor;

        map
    }
}

pub fn draw_map(
    map: Res<Map>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<TerminalTile>>,
) {
    /*
    It is easier to simply reset all the TerminalTile translations, instead of
    trying to figure out which TerminalTile is associated with which Map index.
    */

    // Used to calculate which Map tile needs to be drawn to a TerminalTile
    let mut idx: usize = 0;

    let (x_iterator, y_iterator) = screentiles_iterator();
    let mut query_iter = query.iter_mut();

    for terminal_y in y_iterator {
        for terminal_x in x_iterator.clone() {
            let (mut transform, mut sprite) = query_iter.next().unwrap();

            transform.translation.x = terminal_x as f32;
            transform.translation.y = terminal_y as f32;

            // Convert map Ttile to an index in the textureatlas
            // let (map_x, map_y) = map.idx_xy(idx);
            sprite.index = tiletype_to_spriteindex(map.tiles[idx]);
            idx += 1;
        }
    }
}

fn tiletype_to_spriteindex(tile: TileType) -> u32 {
    match tile {
        TileType::Wall => 35,
        TileType::Floor => 46,
        TileType::DownStairs => 1,
        TileType::UpStairs => 1,
    }
}
