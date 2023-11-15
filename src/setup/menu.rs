use bevy::prelude::*;
mod play;
pub use play::play_button;
use crate::BUTTON;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct Menu;

pub fn menu_start(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: BUTTON.into(),
                        ..default()
                    },
                    PlayButton,
                    Menu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        Menu,
                    ));
                });
        });
}

pub fn menu_despawn(mut commands: Commands, query: Query<(Entity, With<Menu>)>) {
    for (entity, _) in &query {
        commands.entity(entity).despawn();
    }
}
