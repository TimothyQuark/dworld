use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "DWorld".to_string(),
            width: 900.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}