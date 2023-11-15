use bevy::prelude::*;
use crate::{GameState, PlayButton};

pub fn play_button(
    mut next_state: ResMut<NextState<GameState>>, 
    query: Query<(&Interaction, With<PlayButton>)>
){
    let (button_inter, _) = query.single();

    if button_inter == &Interaction::Pressed{
        next_state.set(GameState::InGame);
    }
}