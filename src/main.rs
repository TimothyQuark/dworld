use bevy::prelude::*;

// mod map;
// pub use map::*;

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
