// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use bevy::input::system::exit_on_esc_system;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    winit::cursor::CursorIcon,
};

// use bevy::window::WindowMode;

// mod map_builders;
// use map_builders::build_new_map;

// pub mod components;

// mod systems;
// use systems::{
//     // camera::init_camera,
// //     // input::player_input,
// //     // map::init_map,
// //     // map_indexing::map_indexing,
// //     // monster_ai::monster_ai,
// //     // utilities::print_resources
// //     // player::init_player,
//     terminal::{init_terminal, render_terminal, Terminal},
// };

// mod geometry;
// mod pathfinding;
// mod spawner;
// mod text;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    MainMenu,
    #[default]
    NewGame,
    InGame,
    AwaitingInput,
    MonsterTurn,
}

fn main() {
    // Terminal resource
    // let terminal = Terminal::default();
    // let (screen_width, screen_height) = terminal.get_screen_dim();
    let (screen_width, screen_height) = (1080., 720.);


    // App Builder.
    App::new()
        // Starting State
        // .init_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "DWorld".into(),
                    name: Some("dworld.app".into()),
                    resolution: (screen_width, screen_height).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        // Resources
        // .insert_resource(WindowDescriptor {
        //     title: "DWorld".to_string(),
        //     width: screen_width as f32,
        //     height: screen_height as f32,
        //     vsync: true,
        //     resizable: false,
        //     mode: WindowMode::Windowed,
        //     ..Default::default()
        // })
        // .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(terminal)
        // // Important plugins and debug helpers
        // .add_plugins(DefaultPlugins)
        // // .add_plugin(LogDiagnosticsPlugin::default())
        // // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // // .add_system(print_resources.system())
        // .add_system(exit_on_esc_system)
        // //Startup systems
        // .add_startup_system(init_camera.label("init_camera"))
        // .add_startup_system(init_terminal)
        // .add_startup_system(init_map)
        // .add_startup_system(init_player)
        // .add_system_set(SystemSet::on_enter(AppState::NewGame).with_system(build_new_map))
        // // Normal Systems
        // .add_system(render_terminal)
        // .add_system(map_indexing)
        // .add_system_set(SystemSet::on_enter(AppState::MonsterTurn).with_system(monster_ai))
        // .add_system_set(SystemSet::on_update(AppState::AwaitingInput).with_system(player_input))
        .run();
}
