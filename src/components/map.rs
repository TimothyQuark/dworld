/// Component that describes the kind of tile on a Map
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
}
