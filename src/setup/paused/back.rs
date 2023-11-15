use bevy::prelude::*;
use crate::{BackGame, GameState};

pub fn back_game(
    mut next_state: ResMut<NextState<GameState>>, 
    query: Query<(&Interaction, With<BackGame>)>
){
    let (button_inter, _) = query.single();

    if button_inter == &Interaction::Pressed{
        next_state.set(GameState::AfterPause);
    }
}