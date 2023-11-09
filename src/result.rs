use crate::{Hopper, FINISH_LINE, Score};
use bevy::prelude::*;
use bevy_text::Text;

pub fn cross_finish_line(
    mut hopper_query: Query<(&mut Transform, With<Hopper>)>,
    mut score_query: Query<(&mut Text, With<Score>)>
) {
    let (mut hopper_pos, _hopper) = hopper_query.single_mut();
    let (mut score_text, score) = score_query.single_mut();

    if hopper_pos.translation.y >= FINISH_LINE {
        #[cfg(debug_assertions)]
        {
            dbg!("Cross Line");
        }
        hopper_pos.translation = Vec3::new(0., -170., 0.);

        match score_text.sections.get_mut(0){
            Some(text) => {
                text.value = (text.value.parse::<u32>().unwrap_or(0) + 1).to_string();
            }
            None => ()
        }
    }
}
