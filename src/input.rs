use crate::Hopper;
use bevy::prelude::*;

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Hopper>>) {
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
}
