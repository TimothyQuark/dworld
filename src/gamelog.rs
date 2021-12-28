use bevy::prelude::*;
use std::cmp::min;

use super::BottomSidebar;

pub struct GameLog {
    pub entries: Vec<TextSection>,
}

// Take the entries in the game log resource, and display them on the bottom
// sidebar
pub fn draw_gamelog_system(
    log_resource: Res<GameLog>,                           // 3
    mut query: Query<&mut Text, With<BottomSidebar>>, // 11
) {
    let mut sidebar = query.iter_mut().next().unwrap();
    let idx = min(sidebar.sections.len(), log_resource.entries.len());

    for line in 0..idx {
        sidebar.sections[line] = log_resource.entries[line].clone();
    }

    // What I originally wanted to do, but this does not sit well with Rust
    // for mut sect in &sidebar.sections {
    //     sect.value = log_resource.entries[line].value.clone();
    // }
}

// Creates the GameLog resource, with some default text styles
pub fn init_gamelog_system(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("square.ttf");
    let text_style = TextStyle {
        font,
        // Font size is not in pixels, or there is padding between sections. Hence, smaller than TILESIZE, this was fitted manually
        font_size: 18.1,
        color: Color::WHITE,
    };

    commands.insert_resource(GameLog {
        entries: vec![
            TextSection {
                value: "This should be a new line in the gamelog\n".to_string(),
                style: text_style.clone(),
            };
            3
        ],
    });
}
