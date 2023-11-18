use crate::{GameTime, GameState};
use bevy::prelude::*;

use super::setup::SecondsEnding;

pub fn round_ending(
    mut timer: ResMut<GameTime>,
    time: Res<Time>,
    mut seconds: Query<(&mut Text, With<SecondsEnding>)>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let (mut seconds, _) = seconds.single_mut();
    let game_time = seconds.sections.get_mut(0);
    if timer.0.tick(time.delta()).finished(){
        match game_time {
            Some(num_str) => {
                let num = num_str.value.parse::<f64>().unwrap_or(0.);
                if num < 0.{
                    num_str.value = "0.0".to_string();
                    next_state.set(GameState::Restart)
                }
                num_str.value = format!("{:0.1}", 
                     num - 0.1
                );
            }
            None => (),
        }
    }
}
