use bevy::prelude::*;
use crate::{setup::Exit, GameState, Running};

pub fn to_main(
    mut next_state: ResMut<NextState<GameState>>, 
    button_query: Query<(&Interaction, With<Exit>)>,
    scene_query: Query<(Entity, With<Running>)>,
    mut commands: Commands
){
    let (button_inter, _) = button_query.single();

    if button_inter == &Interaction::Pressed{
        for (entity, _) in &scene_query{
            commands.entity(entity).despawn();
        }

        next_state.set(GameState::MainMenu);
    }
}