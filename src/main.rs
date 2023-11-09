use bevy::prelude::*;

mod setup;
use setup::start;
pub use setup::{FinishLine, Hopper, MiddleLine, Score, StartLine, FINISH_LINE, START_LINE};
mod input;
use input::keyboard_input;
mod result;
use result::cross_finish_line;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start)
        .add_systems(Update, (keyboard_input, cross_finish_line))
        .run();
}
