use bevy::prelude::*;

/// Component used to identify what to draw to terminal (i.e map tiles)
#[derive(Component)]
pub struct TerminalTile;

// struct LeftSidebarText;

/// Component to identify the RightSidebar entity
#[derive(Component)]
pub struct RightSidebar;

/// Component to identify the BottomSidebar entity
#[derive(Component)]
pub struct BottomSidebar;

/// Component to identify the TopSidebar entity
#[derive(Component)]
pub struct TopSidebar;
