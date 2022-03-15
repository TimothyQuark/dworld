use bevy::prelude::*;
/// Component that describes position on the Map
#[derive(Clone, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Component that states an entity is a blocker. Note that this is not used
/// for map tiles, which are not entities (Map is a resource)
#[derive(Component)]
pub struct BlockTile {}
