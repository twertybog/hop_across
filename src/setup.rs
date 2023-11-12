use bevy::prelude::*;

#[derive(Component)]
pub struct Hopper;

#[derive(Component)]
pub struct FinishLine;

#[derive(Component)]
pub struct StartLine;

#[derive(Component)]
pub struct MiddleLine;

#[derive(Component)]
pub struct Score;

pub const FINISH_LINE: f32 = 150.0;
pub const START_LINE: f32 = -150.0;

pub fn start(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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
    ));

    //Score text
    commands.spawn(Text2dBundle {
        text: Text::from_section("Score: ", TextStyle { ..default() }),
        transform: Transform::from_translation(Vec3::new(-230., 200., 0.)),
        ..default()
    });

    //Score points
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(0.to_string(), TextStyle { ..default() }),
            transform: Transform::from_translation(Vec3::new(-200., 200., 0.)),
            ..default()
        },
        Score,
    ));

    //Borders
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(500., 3.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -200., 1.)),
            ..default()
        },))
        .with_children(|child| {
            child.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(3., 356.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-250., 176.75, 1.)),
                ..default()
            });
        })
        .with_children(|child| {
            child.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(3., 356.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(250., 176.75, 1.)),
                ..default()
            });
        });

    //End of word
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.16863, 0.17255, 0.1843),
            custom_size: Some(Vec2::new(50., 356.5)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-276.5, -20., 2.)),
        ..default()
    }).with_children(|child|{
        child.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.16863, 0.17255, 0.1843),
                custom_size: Some(Vec2::new(50., 356.5)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(553., -20., 2.)),
            ..default()
        });
    });
}
