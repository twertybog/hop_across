use bevy::prelude::*;
mod setup;
use setup::start;
pub use setup::{FinishLine, Hopper, MiddleLine, Score, StartLine, FINISH_LINE, START_LINE};
mod input;
use input::keyboard_input;
mod result;
use result::cross_finish_line;
pub use result::HOPPER_START;
mod cars;
use cars::{
    car_hitting_lr, car_hitting_rl, car_moving_lr, car_moving_rl, car_spawning_lr, car_spawning_rl,
};

#[derive(Resource)]
pub struct SpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start)
        .add_systems(
            Update,
            (
                keyboard_input,
                cross_finish_line,
                car_moving_rl,
                car_spawning_rl,
                car_moving_lr,
                car_spawning_lr,
                car_hitting_lr,
                car_hitting_rl,
            ),
        )
        .insert_resource(SpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .run();
}
