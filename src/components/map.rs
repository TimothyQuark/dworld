use bevy::prelude::*;
/// Component that describes position on the Map
#[derive(Clone, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Component that describes the kind of tile on a Map
#[derive(PartialEq, Copy, Clone)]
pub enum MapTileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
}
