use bevy::prelude::*;

// mod map;
// pub use map::*;

mod terminal;
pub use terminal::*;

mod gamelog;
pub use gamelog::*;

fn main() {
    App::build()
        // Window Descriptor needs to exist when the game is build, hence
        // can't simply add to setup system. Same with black background color
        .insert_resource(WindowDescriptor {
            title: "DWorld".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))        
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(setup_terminal.system())
        .add_startup_system(init_gamelog_system.system())
        // .add_system(change_sprite_colors.system())
        .add_system(draw_gamelog_system.system())
        .run();
}

fn setup(mut commands: Commands) {
// Spawn camera and UI Camera bundles
commands.spawn_bundle(OrthographicCameraBundle::new_2d());
commands.spawn_bundle(UiCameraBundle::default());

}
