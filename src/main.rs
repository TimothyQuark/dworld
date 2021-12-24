use bevy::prelude::*;

mod map;
pub use map::*;

const SCREEN_WIDTH: f32 = 200.0;
const SCREEN_HEIGHT: f32 = 200.0;
const TILESIZE: f32 = 20.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "DWorld".to_string(),
            width: SCREEN_HEIGHT,
            height: SCREEN_WIDTH,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(change_sprite_colors.system())
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let texture_handle: Handle<Texture> = assets.load("cp437_20x20_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(20.0, 20.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tile_size = Vec2::splat(TILESIZE);
    // let map_size = Vec2::new(1080.0, 640.0);
    let map_size = Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let half_x = (map_size.x / 2.0) as i32;
    let half_y = (map_size.y / 2.0) as i32;

    let mut sprites = vec![];

    for y in (-half_y + (tile_size[1] / 2.0) as i32..half_y).step_by(tile_size[1] as usize) {
        for x in (-half_x + (tile_size[1] / 2.0) as i32..half_x).step_by(tile_size[0] as usize) {
            sprites.push(SpriteSheetBundle {
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
            });
        }
    }

    commands.spawn_batch(sprites);
}

fn change_sprite_colors(mut query: Query<&mut TextureAtlasSprite>) {
    let mut red = false;
    for mut sprite in &mut query.iter_mut() {
        red = !red;
        if red {
            sprite.color = Color::RED
        } else {
            sprite.color = Color::BLUE
        }
    }
}
