// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;

mod map_builders;
use map_builders::build_new_map;

pub mod components;

mod gamelog;
pub use gamelog::*;

mod text;
pub use text::*;

mod utilities;
pub use utilities::*;

mod geometry;

mod systems;
use systems::{
    map::init_map,
    player::init_player,
    terminal::{init_terminal, render_terminal, Terminal},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    NewGame,
    InGame,
}

fn main() {
    /*
    Window Descriptor needs to exist when the game is build, hence
    can't simply add to setup system. Same with black background color,
    needs to be added a resource ahead of time.

    Initial state: Main Menu
    */

    // Terminal resource
    let terminal = Terminal::default();
    let (screen_width, screen_height) = terminal.get_screen_dim();

    App::new()
        .add_state(AppState::NewGame)
        .insert_resource(WindowDescriptor {
            title: "DWorld".to_string(),
            width: screen_width as f32,
            height: screen_height as f32,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(terminal)
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(init_camera.system().label("init_camera"))
        .add_startup_system(init_terminal.system())
        .add_startup_system(init_map.system())
        .add_startup_system(init_player.system())
        // .add_startup_system(setup_terminal.system().after("setup"))
        // .add_startup_system(init_gamelog_system.system().after("setup"))
        // .add_system(print_resources.system())
        // .add_system(change_sprite_colors.system())
        // .add_system(draw_gamelog_system.system())
        // .add_system_set(new_map_system.system())
        .add_system_set(SystemSet::on_enter(AppState::NewGame).with_system(build_new_map))
        .add_system(render_terminal.system())
        .run();
}

fn init_camera(mut commands: Commands) {
    println!("Initialize camera bundles");

    // Spawn camera and UI Camera bundles
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
