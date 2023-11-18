use crate::{Hopper, FINISH_LINE, Score, ScoreBoard};
use bevy::prelude::*;
pub const HOPPER_START: Vec3 = Vec3::new(0., -170., 0.);

pub fn cross_finish_line(
    mut hopper_query: Query<(&mut Transform, With<Hopper>)>,
    mut score: ResMut<Score>,
    mut score_query: Query<(&mut Text, With<ScoreBoard>)>
) {
    let (mut hopper_pos, _hopper) = hopper_query.single_mut();
    let (mut score_text, _) = score_query.single_mut();
    if hopper_pos.translation.y >= FINISH_LINE {
        #[cfg(debug_assertions)]
        {
            dbg!("Cross Line");
        }
        hopper_pos.translation = HOPPER_START;

        score.0 += 1;

        match score_text.sections.get_mut(0){
            Some(text) => {
                text.value = format!("Score: {}", score.0);
            },
            None => ()
        }
    }
}
