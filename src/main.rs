use bevy::prelude::*;

mod map;
pub use map::*;

mod terminal;
pub use terminal::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "DWorld".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_terminal.system())
        .add_system(change_sprite_colors.system())
        .run();
}

fn change_sprite_colors(mut query: Query<&mut TextureAtlasSprite>) {
    let mut red = false;
    for mut sprite in &mut query.iter_mut() {
        // println!("Found a sprite to draw!");
        red = !red;
        if red {
            sprite.color = Color::RED;
            sprite.index = 4;
        } else {
            sprite.color = Color::BLUE;
        }
    }
}
