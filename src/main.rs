// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy::window::WindowMode;

mod map_builders;
use map_builders::build_new_map;

pub mod components;

mod systems;
use systems::{
    camera::init_camera,
    input::player_input,
    map::init_map,
    monster_ai::monster_ai,
    // utilities::print_resources
    player::init_player,
    terminal::{init_terminal, render_terminal, Terminal},
    update_blockers::map_indexing,
};

mod text;

mod geometry;

mod spawner;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    NewGame,
    InGame,
    AwaitingInput,
    MonsterTurn,
}

fn main() {
    // Terminal resource
    let terminal = Terminal::default();
    let (screen_width, screen_height) = terminal.get_screen_dim();

    // App Builder.
    App::new()
        .add_state(AppState::NewGame)
        // Resource
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
        // Important plugins and debug helpers
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(print_resources.system())
        .add_system(exit_on_esc_system.system())
        //Startup systems
        .add_startup_system(init_camera.system().label("init_camera"))
        .add_startup_system(init_terminal.system())
        .add_startup_system(init_map.system())
        .add_startup_system(init_player.system())
        .add_system_set(SystemSet::on_enter(AppState::NewGame).with_system(build_new_map))
        // Normal Systems
        .add_system(render_terminal.system())
        // .add_system(player_input.system())
        .add_system_set(SystemSet::on_update(AppState::AwaitingInput).with_system(player_input))
        .add_system_set(SystemSet::on_update(AppState::MonsterTurn).with_system(monster_ai))
        .add_system(map_indexing.system())
        .run();
}
