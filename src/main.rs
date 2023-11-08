use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position(Vec3);

#[derive(Component)]
struct Hopper;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, (keyboard_input, ))
    .run();
}

fn setup(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(15.0, 15.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    },
    Hopper
));
}


fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Hopper>> ){
    let mut position = query.single_mut();

    if keys.pressed(KeyCode::W){
        position.translation.y += 2.0;
        println!("Pressed W!");
    }
    if keys.pressed(KeyCode::S){
        position.translation.y -= 2.0;
        println!("Pressed S!");
    }
    if keys.pressed(KeyCode::A){
        position.translation.x -= 2.0;
        println!("Pressed A!");
    }
    if keys.pressed(KeyCode::D){
        position.translation.x += 2.0;
        println!("Pressed D!");
    }
}
