use bevy::prelude::*;
use std::default;
mod setup;
use setup::{
    back_game, camera_setup, ingame_start, menu_despawn, menu_start, pause_menu, PlayButton,
    Running,
};
pub use setup::{
    despawn_restart, pause_despawn, play_button, restart_game, restart_menu, to_main, BackGame,
    FinishLine, Hopper, MiddleLine, ScoreBoard, StartLine, FINISH_LINE, START_LINE,
};
mod input;
use input::{keyboard_input, pause_input};
mod result;
use result::cross_finish_line;
pub use result::HOPPER_START;
mod cars;
use cars::{
    car_despawning_lr, car_despawning_rl, car_hitting_lr, car_hitting_rl, car_moving_lr,
    car_moving_rl, car_spawning_lr, car_spawning_rl,
};
mod border;
use border::border_collision;
mod time;
use time::round_ending;

pub const BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Resource)]
pub struct SpawnTimer(Timer);

#[derive(Resource)]
pub struct GameTime(Timer);

#[derive(Resource)]
pub struct Score(u64);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    MainMenu,
    InGame,
    GamePause,
    AfterPause,
    #[default]
    Restart,
}

fn main() {
    let ingame = (
        keyboard_input,
        cross_finish_line,
        car_moving_rl,
        car_spawning_rl,
        car_moving_lr,
        car_spawning_lr,
        car_hitting_lr,
        car_hitting_rl,
        car_despawning_lr,
        car_despawning_rl,
        border_collision,
        round_ending,
    );

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, camera_setup)
        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::MainMenu), menu_start)
        .add_systems(OnExit(GameState::MainMenu), menu_despawn)
        .add_systems(Update, (play_button).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnEnter(GameState::InGame), ingame_start)
        .add_systems(OnEnter(GameState::GamePause), pause_menu)
        .add_systems(OnExit(GameState::GamePause), pause_despawn)
        .add_systems(OnEnter(GameState::Restart), restart_menu)
        .add_systems(OnExit(GameState::Restart), despawn_restart)
        .add_systems(Update, ingame.run_if(in_state(GameState::InGame)))
        .insert_resource(SpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .add_systems(
            Update,
            (back_game, to_main, pause_input).run_if(in_state(GameState::GamePause)),
        )
        .add_systems(Update, ingame.run_if(in_state(GameState::AfterPause)))
        .add_systems(
            Update,
            (to_main, restart_game).run_if(in_state(GameState::Restart)),
        )
        .insert_resource(GameTime(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(Score(0))
        .run();
}
