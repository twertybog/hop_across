use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;
use crate::SpawnTimer;

const LR_POSITION: [f32; 5] = [24.0, 48.0, 72.0, 96.0, 120.0];
const RL_POSITION: [f32; 5] = [-24.0, -48.0, -72.0, -96.0, -120.0];

#[derive(Component)]
pub struct LRCar;

#[derive(Component)]
pub struct RLCar;

pub fn car_spawning_rl(
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>
){
    if !timer.0.tick(Duration::from_secs_f32(0.2)).finished(){
        return;
    }

    let mut rng = rand::thread_rng();
    let position = RL_POSITION[rng.gen_range(0..5)];

    commands.spawn((
        SpriteBundle{
            sprite: Sprite{
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(30., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-320., position, 0.)),
            ..default()
        },
        RLCar
    ));

    #[cfg(debug_assertions)]{
        dbg!("Top car spawned!");
    }
}

pub fn car_spawning_lr(
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>
){
    if !timer.0.tick(Duration::from_secs_f32(0.2)).finished(){
        return;
    }

    let mut rng = rand::thread_rng();
    let position = LR_POSITION[rng.gen_range(0..5)];

    commands.spawn((
        SpriteBundle{
            sprite: Sprite{
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(30., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(320., position, 0.)),
            ..default()
        },
        LRCar
    ));

    #[cfg(debug_assertions)]{
        dbg!("Bottom car spawned!");
    }
}

pub fn car_moving_rl(mut query: Query<(&mut Transform, With<RLCar>)>){
    for (mut car_position_rl, _car_rl) in query.iter_mut(){
        car_position_rl.translation.x += 2.2;
    }
}

pub fn car_moving_lr(mut query: Query<(&mut Transform, With<LRCar>)>){
    for (mut car_position_lr, _car_lr) in query.iter_mut(){
        car_position_lr.translation.x -= 2.2;
    }
}