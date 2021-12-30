use bevy::prelude::*;

// This module contains useful boilerplate code and structs for the text in DWorld

/// Default Font used by Dworld
// pub struct DefaultFont(pub Handle<Font>);

/// Default TextStyle used by DWorld
pub struct DefaultTextStyle(pub TextStyle);

/// Loads the default font in DWorld, and returns the default text style
pub fn default_textstyle(assets: Res<AssetServer>) -> TextStyle {
    let font = assets.load("square.ttf");
    let text_style = TextStyle {
        font,
        // Font size is not in pixels, or there is padding between sections. Hence, smaller than TILESIZE, this was fitted manually
        font_size: 18.1,
        color: Color::WHITE,
    };

    text_style
}
