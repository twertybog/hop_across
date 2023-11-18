use bevy::prelude::*;

use crate::Score;

#[derive(Component)]
pub struct Hopper;

#[derive(Component)]
pub struct FinishLine;

#[derive(Component)]
pub struct StartLine;

#[derive(Component)]
pub struct MiddleLine;

#[derive(Component)]
pub struct Running;

#[derive(Component)]
pub struct SecondsEnding;

#[derive(Component)]
pub struct ScoreBoard;

pub const FINISH_LINE: f32 = 150.0;
pub const START_LINE: f32 = -150.0;

pub fn ingame_start(mut commands: Commands, mut score: ResMut<Score>) {
    score.0 = 0;
    //Hopper
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -170., 0.)),
            ..default()
        },
        Hopper,
        Running,
    ));

    //Finish line
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(500., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., FINISH_LINE, -1.)),
            ..default()
        },
        FinishLine,
        Running,
    ));

    //Start Line
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(500., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., START_LINE, -1.)),
            ..default()
        },
        StartLine,
        Running,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(500., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
            ..default()
        },
        MiddleLine,
        Running,
    ));

    //Score text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(format!("Score: {}", score.0), TextStyle { ..default() }),
            transform: Transform::from_translation(Vec3::new(-230., 200., 0.)),
            ..default()
        },
        Running,
        ScoreBoard
    ));

    //Seconds
    commands.spawn((Text2dBundle{
        text: Text::from_section("60.0", TextStyle{..default()}),
        transform: Transform::from_translation(Vec3::new(0., 200., 0.)),
        ..default()
    }, Running, SecondsEnding));

    //Borders
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(500., 3.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., -200., 1.)),
                ..default()
            },
            Running,
        ))
        .with_children(|child| {
            child.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(3., 356.5)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(-250., 176.75, 1.)),
                    ..default()
                },
                Running,
            ));
        })
        .with_children(|child| {
            child.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(3., 356.5)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(250., 176.75, 1.)),
                    ..default()
                },
                Running,
            ));
        });

    //End of word
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.16863, 0.17255, 0.1843),
                    custom_size: Some(Vec2::new(50., 356.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-276.5, -20., 2.)),
                ..default()
            },
            Running,
        ))
        .with_children(|child| {
            child.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.16863, 0.17255, 0.1843),
                        custom_size: Some(Vec2::new(50., 356.5)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(553., -20., 2.)),
                    ..default()
                },
                Running,
            ));
        });
}
