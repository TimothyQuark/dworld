use bevy::prelude::*;

use crate::components::rendering::TerminalTile;
use crate::text::{default_textstyle, DefaultTextStyle};

/// Terminal resource, contains all important information about the
/// Game Window, such as screen dimensions, screen tile dimensions etc.
pub struct Terminal {
    // TODO: Tile is currently a square, change to be a rectangle
    tile_size: i32,
    screen_width: i32,
    screen_height: i32,
    terminal_tiles: Vec<(usize, Color)>, // Vec<(SpriteIndex, Color)
}

impl Default for Terminal {
    /// Returns default Terminal resource.
    ///
    /// Tile size: 20 pixels
    /// Screen width: 1080 pixels
    /// Screen height: 720 pixels
    fn default() -> Self {
        let tile_size = 20;
        let screen_width = 1080;
        let screen_height = 720;

        Self {
            tile_size: tile_size,
            screen_width: screen_width,
            screen_height: screen_height,
            terminal_tiles: vec![
                (46, Color::BLUE);
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
        }
    }
}

impl Terminal {
    #![allow(dead_code)]
    /// Create Terminal resource with custom settings
    fn new(tile_size: i32, screen_width: i32, screen_height: i32) -> Self {
        Self {
            tile_size: tile_size,
            screen_width,
            screen_height,
            terminal_tiles: vec![
                (46, Color::BLUE);
                (screen_width / tile_size * screen_height / tile_size) as usize
            ],
        }
    }
    /// Returns screen dimensions, in pixels.
    ///
    /// (screen_width, screen_height)

    pub fn get_screen_dim(&self) -> (i32, i32) {
        (self.screen_width, self.screen_height)
    }

    /// Returns terminal dimensions, in tiles
    ///
    /// (terminal_width, terminal_height)
    pub fn get_terminal_dim(&self) -> (i32, i32) {
        (
            self.screen_width / self.tile_size,
            self.screen_height / self.tile_size,
        )
    }
}

pub fn init_terminal_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    terminal: Res<Terminal>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Note that this system does not actually create the terminal resource,
    // that is done in the main app.

    println!("Initializing the terminal");

    // println!("{}", terminal.get_screen_dim().0);

    // Load the default tile sheet
    // Load sprite sheet into a texture atlas
    let texture_handle = assets.load("cp437_20x20_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(20.0, 20.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Load the default font and text style, and add as a resource.
    // Note that resources may not be accessible to startup systems.
    let default_text_style = default_textstyle(assets);
    commands.insert_resource(DefaultTextStyle(default_text_style));

    // Spawn the Terminal Tile entities, which will be used to draw terminal contents
    // (terminal_tiles) to the screen
    // Bevy uses coordinate system where center of screen is (0,0), also
    // sprite translation is center of sprite. Need lots of awful
    // coordinate shifting
    let x_min = (-1 * terminal.screen_width / 2) + terminal.tile_size / 2;
    let x_max = (terminal.screen_width) / 2;
    let y_min = (-1 * terminal.screen_height / 2) + terminal.tile_size / 2;
    let y_max = (terminal.screen_height) / 2;

    let mut idx: usize = 0;
    for x in (x_min..x_max).step_by(terminal.tile_size as usize) {
        for y in (y_min..y_max).step_by(terminal.tile_size as usize) {
            // println!("x:{}, y: {}", x, y);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        // Translation is middle of sprite, hence iterator uses stuff like tile_size / 2.0 etc
                        translation: Vec3::new(x as f32, y as f32, 0.0),
                        scale: Vec3::splat(1.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite {
                        color: Color::PINK,
                        index: 10,
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas_handle.clone(),
                    ..Default::default()
                })
                .insert(TerminalTile { idx });

            idx += 1;
        }
    }
}

/// System that renders the terminal every frame.
pub fn render_terminal_system(
    // mut commands: Commands,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite, &TerminalTile), With<TerminalTile>>,
    terminal: Res<Terminal>,
) {
    let query_iter = query.iter_mut();
    for tile in query_iter {
        let (_, mut sprite, tile_component) = tile;
        sprite.index = terminal.terminal_tiles[tile_component.idx].0;
        sprite.color = terminal.terminal_tiles[tile_component.idx].1;

        // let (terminal_width, terminal_height) = terminal.get_terminal_dim();
        // let x = (tile.0.translation.x + (terminal.tile_size / 2) as f32) / (terminal.tile_size as f32);
        // let y = (tile.0.translation.y + (terminal.tile_size / 2) as f32) / (terminal.tile_size as f32);

        // println!(
        //     "t_width: {}, t_height: {}, tran_x: {}, tran_y: {}, x:{}, y: {}",
        //     terminal_width, terminal_height, tile.0.translation.x, tile.0.translation.y, x, y
        // );
        // let terminal_tile =
    }
}
