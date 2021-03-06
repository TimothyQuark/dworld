// use std::iter::StepBy;

// use bevy::prelude::*;

// use super::components::rendering::*;

// use super::default_textstyle;

// // Systems that deal with the terminal/game window itself.

// // Sizes of various important areas of the terminal.
// // All these numbers are in pixels.
// // Note that numbers should be divisible by TILESIZE
// pub const TILESIZE: f32 = 20.0;
// pub const SCREEN_WIDTH: f32 = 1080.0;
// pub const SCREEN_HEIGHT: f32 = 720.0;
// // const LEFT_SIDEBAR: f32 = 0.0;
// const RIGHT_SIDEBAR: f32 = 280.0;
// const BOTTOM_SIDEBAR: f32 = 220.0;
// const TOP_SIDEBAR: f32 = 20.0;

// /*
// Note to self: it is basically impossible to see inside starting systems with
// VSC debugging, all the variables are either optimized away or can't be found.
// Also, variables like assets and texture_atlases are duplicated dozens of times...
// */

// /// Calculates translation of TerminalTiles.
// /// (x_iterator, y_iterator)
// pub fn screentiles_iterator() -> (StepBy<std::ops::Range<i32>>, StepBy<std::ops::Range<i32>>) {
//     let tile_size = Vec2::splat(TILESIZE);

//     let half_x = (SCREEN_WIDTH / 2.0) as i32;
//     let half_y = (SCREEN_HEIGHT / 2.0) as i32;

//     let y_iterator = (-half_y + (BOTTOM_SIDEBAR + tile_size[1] / 2.0) as i32
//         ..half_y - TOP_SIDEBAR as i32)
//         .step_by(tile_size[1] as usize);
//     let x_iterator = (-half_x + (tile_size[0] / 2.0) as i32..half_x - RIGHT_SIDEBAR as i32)
//         .step_by(tile_size[0] as usize);

//     (x_iterator, y_iterator)
// }

// /**
// System which initializes the game window, and creates the various entities associated
// with it, such as the sidebar text.
// */
// pub fn setup_terminal(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     println!("Setting up the terminal");

//     // Load sprite sheet into a texture atlas
//     let texture_handle = assets.load("cp437_20x20_transparent.png");
//     let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(20.0, 20.0), 16, 16);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);

//     // Load font. Cannot use DefaultFont because resources cannot be accessed
//     // by startup systems
//     let text_style = default_textstyle(assets);

//     // Using square tiles for now, but might not always be the case
//     // Use this instead of TILESIZE
//     let tile_size = Vec2::splat(TILESIZE);

//     // Bevy has (0,0) coordinate in middle of the window, not in some corner. This is that offset
//     // REMEMBER that this makes a lot of calculations fairly complicated, hence we have stuff like tile_size / 2.0 etc to shift everything up
//     let half_x = (SCREEN_WIDTH / 2.0) as i32;
//     let half_y = (SCREEN_HEIGHT / 2.0) as i32;

//     let (x_iterator, y_iterator) = screentiles_iterator();

//     /*
//     Create entities, all with a SpriteSheetBundle and TerminalTile components. These entities
//     are the inidividual tiles you see on the terminal. Defaulted to hearts, so you can
//     tell if it didn't load correctly.
//     The translation (position) of these entities should never change, only
//     color and index
//     */

//     /*
//     PROBLEM WITH MAP VISUALIZATION: Currently map size is tied to SCREEN_DIMENSIONS, but
//     the map itself does not fill the entire screen. Need to decouple location of map tiles
//     from the system that draws the tiles to the terminal.
//     */

//     for y in y_iterator {
//         for x in x_iterator.clone() {
//             // SpriteSheetBundle is a bundle of components, not a component itself.
//             commands
//                 .spawn_bundle(SpriteSheetBundle {
//                     transform: Transform {
//                         // Translation is middle of sprite, hence iterator uses stuff like tile_size / 2.0 etc
//                         translation: Vec3::new(x as f32, y as f32, 0.0),
//                         scale: Vec3::splat(1.0),
//                         ..Default::default()
//                     },
//                     sprite: TextureAtlasSprite {
//                         color: Color::BLUE,
//                         index: 6,
//                         ..Default::default()
//                     },
//                     texture_atlas: texture_atlas_handle.clone(),
//                     ..Default::default()
//                 })
//                 .insert(TerminalTile);
//         }
//     }

//     // Spawn text for top sidebar
//     // TODO: Maybe using TextBundle might be better, because it allows for limiting size of the text box, also set padding.
//     // Can use a Rect to set the area the textbox fills.
//     // See https://github.com/bevyengine/bevy/blob/latest/examples/ui/text_debug.rs for example
//     commands
//         .spawn_bundle(Text2dBundle {
//             text: Text::with_section(
//                 "Weather: Cold   Time: 10:51 (Default Text)",
//                 text_style.clone(),
//                 TextAlignment {
//                     vertical: VerticalAlign::Center,
//                     horizontal: HorizontalAlign::Left,
//                 },
//             ),
//             transform: Transform {
//                 // For text, this translation sets the x as the leftmost limit, and y is the center of the first line.
//                 translation: Vec3::new(-half_x as f32, half_y as f32 - (tile_size[1] / 2.0), 0.0),
//                 scale: Vec3::ONE,
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(TopSidebar);

//     // Spawn text for bottom sidebar. Sections added so that you can accesss
//     // them later (also don't want to add more sections during the game itself)
//     commands
//         .spawn_bundle(Text2dBundle {
//             text: Text {
//                 sections: vec![
//                     TextSection {
//                         value: "------------------------------------- Add log text here\n".to_string(),
//                         style: text_style.clone(),
//                     };
//                     // Number of sections should be as many lines as in the log
//                     (BOTTOM_SIDEBAR / TILESIZE) as usize
//                 ],
//                 alignment: TextAlignment {
//                     vertical: VerticalAlign::Top,
//                     horizontal: HorizontalAlign::Left,
//                 },
//             },
//             transform: Transform {
//                 translation: Vec3::new(-half_x as f32, (-half_y as f32) + BOTTOM_SIDEBAR, 0.0),
//                 scale: Vec3::ONE,
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(BottomSidebar);

//     // Spawn text for right sidebar. Sections added so that you can accesss
//     // them later (also don't want to add more sections during the game itself)
//     commands
//         .spawn_bundle(Text2dBundle {
//             text: Text {
//                 sections: vec![
//                     TextSection {
//                         value: "This is a line on the right side (Default text)\n".to_string(),
//                         style: text_style,
//                     };
//                     // Number of sections should be as many lines as in the log
//                     ((SCREEN_HEIGHT - BOTTOM_SIDEBAR - TILESIZE) / TILESIZE) as usize
//                 ],
//                 alignment: TextAlignment {
//                     vertical: VerticalAlign::Top,
//                     horizontal: HorizontalAlign::Left,
//                 },
//             },
//             transform: Transform {
//                 // Start one line below the top sidebar so they do not overlap
//                 translation: Vec3::new(
//                     half_x as f32 - RIGHT_SIDEBAR,
//                     half_y as f32 - TOP_SIDEBAR,
//                     0.0,
//                 ),
//                 scale: Vec3::ONE,
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(RightSidebar);
// }

// // pub fn change_sprite_colors(mut query: Query<&mut TextureAtlasSprite>) {
// //     let mut red = false;
// //     for mut sprite in &mut query.iter_mut() {
// //         // println!("Found a sprite to draw!");
// //         red = !red;
// //         if red {
// //             sprite.color = Color::RED;
// //             sprite.index = 4;
// //         } else {
// //             sprite.color = Color::BLUE;
// //         }
// //     }
// // }
