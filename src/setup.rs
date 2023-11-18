use bevy::prelude::*;
mod ingame;
mod menu;
mod paused;
mod restart;
pub use menu::{menu_start, play_button, PlayButton, menu_despawn};
pub use ingame::{
    ingame_start, FinishLine, Hopper, MiddleLine, ScoreBoard,
    StartLine, FINISH_LINE, START_LINE, Running, SecondsEnding
};
pub use paused::{pause_menu, back_game, pause_despawn, BackGame, to_main, Exit};
pub use restart::{restart_menu, despawn_restart, restart_game};

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


