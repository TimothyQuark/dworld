use bevy::prelude::*;

// Sizes of various important areas of the terminal.
// All these numbers are in pixels.
// Note that all numbers should be divisible by TILESIZE
pub const TILESIZE: f32 = 20.0;
pub const SCREEN_WIDTH: f32 = 1080.0;
pub const SCREEN_HEIGHT: f32 = 720.0;


const LEFT_SIDEBAR: f32 = 0.0;
const RIGHT_SIDEBAR: f32 = 280.0;
const BOTTOM_SIDEBAR: f32 = 220.0;
const TOP_SIDEBAR: f32 = 20.0;

// Components used
struct TerminalTile;

struct LeftSidebarText;
struct RightSidebarText;
struct BottomSidebarText;
struct TopSidebarText;

// A system which initializes everything for the DWorld Window, such as
// the terminal, the sidebars
pub fn setup_terminal(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn camera and UI Camera bundles
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Load sprite sheet into a texture atlas
    let texture_handle: Handle<Texture> = assets.load("cp437_20x20_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(20.0, 20.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Load font
    let font = assets.load("square.ttf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };

    // Using square tiles for now, but might not always be the case
    let tile_size = Vec2::splat(TILESIZE);

    // Bevy has (0,0) coordinate in middle of the window, not in some corner. This is that offset
    let half_x = (SCREEN_WIDTH / 2.0) as i32;
    let half_y = (SCREEN_HEIGHT / 2.0) as i32;

    let y_iterator = (-half_y + (tile_size[1] / 2.0 + BOTTOM_SIDEBAR) as i32
        ..half_y - TOP_SIDEBAR as i32)
        .step_by(tile_size[1] as usize);
    let x_iterator = (-half_x + (tile_size[0] / 2.0 + LEFT_SIDEBAR) as i32
        ..half_x - RIGHT_SIDEBAR as i32)
        .step_by(tile_size[0] as usize);

    // Create entities, all with a SpriteSheetBundle and TerminalTile components. These entities
    // are the inidividual tiles you see on the terminal. Defaulted to hearts, so you can
    // tell if it didn't load correctly.
    // The translation (position) of these entities should never change, only
    // color and index
    for y in y_iterator {
        for x in x_iterator.clone() {
            // SpriteSheetBundle is a bundle itself, hence can't just use a tuple for spawn_bundle function
            commands
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(x as f32, y as f32, 0.0),
                        scale: Vec3::splat(1.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite {
                        color: Color::BLUE,
                        index: 6,
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas_handle.clone(),
                    ..Default::default()
                })
                .insert(TerminalTile);
        }
    }

    // Spawn text for top sidebar
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Weather: Cold   Time: 10:51",
                text_style.clone(),
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Right,
                },
            ),
            transform: Transform {
                translation: Vec3::new(-half_x as f32, half_y as f32 - (TILESIZE / 2.0), 0.0),
                scale: Vec3::ONE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TopSidebarText);

    // Spawn text for bottom sidebar. Sections added so that you can accesss
    // them later (also don't want to add more sections during the game itself)
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                
                sections: vec![
                    TextSection {
                        value: "This is a line in the log\n".to_string(),
                        style: text_style.clone(),
                    };
                    // Number of sections should be as many lines as in the log
                    (BOTTOM_SIDEBAR / TILESIZE) as usize
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Right,
                },
            },
            transform: Transform {
                translation: Vec3::new(-half_x as f32, (-half_y as f32) + BOTTOM_SIDEBAR, 0.0),
                scale: Vec3::ONE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BottomSidebarText);

    // Spawn text for right sidebar. Sections added so that you can accesss
    // them later (also don't want to add more sections during the game itself)
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                
                sections: vec![
                    TextSection {
                        value: "This is a line on the right side\n".to_string(),
                        style: text_style.clone(),
                    };
                    // Number of sections should be as many lines as in the log
                    (RIGHT_SIDEBAR / TILESIZE) as usize
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Right,
                },
            },
            transform: Transform {
                translation: Vec3::new((half_x as f32 - RIGHT_SIDEBAR), (-half_y as f32) + BOTTOM_SIDEBAR, 0.0),
                scale: Vec3::ONE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RightSidebarText);
}