use bevy::prelude::*;
use crate::Hopper;

pub fn border_collision(mut query: Query<(&mut Transform, With<Hopper>)>){
    let (mut hooper_pos, _) = query.single_mut();

    if hooper_pos.translation.x <= -240.{
        hooper_pos.translation.x += 2.0;
    }

    if hooper_pos.translation.x >= 240.{
        hooper_pos.translation.x -= 2.0;
    }

    if hooper_pos.translation.y <= -190.0{
        hooper_pos.translation.y += 2.0;
    }
}