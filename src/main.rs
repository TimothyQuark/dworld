use bevy::prelude::*;

// mod map;
// pub use map::*;

mod terminal;
pub use terminal::*;

mod gamelog;
pub use gamelog::*;

mod text;
pub use text::*;

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
        .add_startup_system(setup.system().label("setup"))
        .add_startup_system(setup_terminal.system().after("setup"))
        .add_startup_system(init_gamelog_system.system().after("setup"))
        // .add_system(change_sprite_colors.system())
        .add_system(draw_gamelog_system.system())
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    println!("Basic setup system started");

    // Spawn camera and UI Camera bundles
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Load the default font and text style. Note that resources are not accessible to
    // startup systems.
    let font_handle = assets.load("square.ttf");
    let text_style = TextStyle {
        font: font_handle.clone(),
        // Font size is not in pixels, or there is padding between sections. Hence, smaller than TILESIZE, this was fitted manually
        font_size: 18.1,
        color: Color::WHITE,
    };
    commands.insert_resource(DefaultFont(font_handle));    
    commands.insert_resource(DefaultTextStyle(text_style));
}


