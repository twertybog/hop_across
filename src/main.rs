use bevy::prelude::*;

#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Hopper;

#[derive(Component)]
struct FinishLine;

#[derive(Component)]
struct StartLine;

#[derive(Component)]
struct MiddleLine;

const FINISH_LINE: f32 = 150.0;
const START_LINE: f32 = -150.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, cross_finish_line))
        .run();
}

fn setup(mut commands: Commands) {
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
        SpriteBundle{
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(500., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., FINISH_LINE, -1.)),
            ..default()
        },
        FinishLine
    ));

    //Start Line
    commands.spawn((
        SpriteBundle{
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(500., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., START_LINE, -1.)),
            ..default()
        },
        StartLine
    ));

    commands.spawn((
        SpriteBundle{
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(500., 7.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
            ..default()
        },
        MiddleLine
    ));
    
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Hopper>>) {
    let mut position = query.single_mut();

    if keys.pressed(KeyCode::W) {
        position.translation.y += 2.0;

        #[cfg(debug_assertions)]{
            dbg!("Pressed W!");
        }
    }
    if keys.pressed(KeyCode::S) {
        position.translation.y -= 2.0;

        #[cfg(debug_assertions)]{
            dbg!("Pressed S!");
        }
    }
    if keys.pressed(KeyCode::A) {
        position.translation.x -= 2.0;
        
        #[cfg(debug_assertions)]{
            dbg!("Pressed A!");
        }
    }
    if keys.pressed(KeyCode::D) {
        position.translation.x += 2.0;
        
        #[cfg(debug_assertions)]{
            dbg!("Pressed D!");
        }
    }
}

fn cross_finish_line(
    mut hopper_query: Query<(&mut Transform, With<Hopper>)>,
){
    let (mut hopper_pos, hopper) = hopper_query.single_mut();

    if hopper_pos.translation.y >= FINISH_LINE{
        #[cfg(debug_assertions)]{
            dbg!("Cross Line");
        }
        hopper_pos.translation = Vec3::new(0., -170., 0.);
    }
}
