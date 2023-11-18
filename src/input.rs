use crate::{GameState, Hopper};
use bevy::prelude::*;

pub fn keyboard_input(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Hopper>>,
) {
    let mut position = query.single_mut();

    if keys.pressed(KeyCode::W) {
        position.translation.y += 2.0;

        #[cfg(debug_assertions)]
        {
            dbg!("Pressed W!");
        }
    }
    if keys.pressed(KeyCode::S) {
        position.translation.y -= 2.0;

        #[cfg(debug_assertions)]
        {
            dbg!("Pressed S!");
        }
    }
    if keys.pressed(KeyCode::A) {
        position.translation.x -= 2.0;

        #[cfg(debug_assertions)]
        {
            dbg!("Pressed A!");
        }
    }
    if keys.pressed(KeyCode::D) {
        position.translation.x += 2.0;

        #[cfg(debug_assertions)]
        {
            dbg!("Pressed D!");
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::GamePause);
        #[cfg(debug_assertions)]
        {
            dbg!("Pressed Esc!");
        }
    }
}

pub fn pause_input(mut next_state: ResMut<NextState<GameState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::AfterPause);
    }
}
