use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub mod map_builders;
use map_builders::new_map_system;

pub mod components;

mod map;
use bevy::window::WindowMode;
pub use map::*;

mod terminal;
pub use terminal::*;

mod gamelog;
pub use gamelog::*;

mod text;
pub use text::*;

mod utilities;
pub use utilities::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    // MainMenu,
    NewMap,
    InGame,
}

fn main() {
    App::new()
        .add_state(AppState::NewMap)
        // Window Descriptor needs to exist when the game is build, hence
        // can't simply add to setup system. Same with black background color
        .insert_resource(WindowDescriptor {
            title: "DWorld".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system().label("setup"))
        .add_startup_system(setup_terminal.system().after("setup"))
        .add_startup_system(init_gamelog_system.system().after("setup"))
        .add_system(bevy::input::system::exit_on_esc_system.system())
        // .add_system(print_resources.system())
        // .add_system(change_sprite_colors.system())
        .add_system(draw_gamelog_system.system())
        .add_system(draw_map_system.system())
        // .add_system_set(new_map_system.system())
        .add_system_set(SystemSet::on_enter(AppState::NewMap).with_system(new_map_system))
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    println!("Basic setup system started");

    // Spawn camera and UI Camera bundles
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Load the default font and text style. Note that resources are not accessible to
    // startup systems.
    let default_text_style = default_textstyle(assets);
    // commands.insert_resource(DefaultFont(font_handle));
    commands.insert_resource(DefaultTextStyle(default_text_style));

    // Add a default map resource
    commands.insert_resource(Map::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
}
