use bevy::prelude::*;
mod ingame;
mod menu;
mod paused;
pub use menu::{menu_start, play_button, PlayButton, menu_despawn};
pub use ingame::{
    ingame_start, FinishLine, Hopper, MiddleLine, Score, StartLine, FINISH_LINE, START_LINE, Running
};
pub use paused::{pause_menu, back_game, pause_despawn, BackGame, to_main, Exit};

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


