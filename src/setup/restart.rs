use crate::{setup::Exit, GameState, BUTTON};
use bevy::prelude::*;

use super::Running;

#[derive(Component)]
pub struct RestartMenu;

#[derive(Component)]
pub struct Restart;

pub fn restart_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                background_color: Color::rgba(0.1, 0.1, 0.1, 0.6).into(),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            RestartMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: BUTTON.into(),
                        ..default()
                    },
                    RestartMenu,
                    Restart,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Restart",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                        RestartMenu,
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: BUTTON.into(),
                        ..default()
                    },
                    RestartMenu,
                    Exit,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Exit",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                        RestartMenu,
                    ));
                });
        });
}

pub fn restart_game(
    button_query: Query<(&Interaction, With<Restart>)>,
    scene_query: Query<(Entity, With<Running>)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (restart, _) = button_query.single();

    if restart == &Interaction::Pressed {
        for (entity, _) in &scene_query {
            commands.entity(entity).despawn();
        }
        next_state.set(GameState::InGame);
    }
}

pub fn despawn_restart(query: Query<(Entity, With<RestartMenu>)>, mut commands: Commands) {
    for (entity, _) in &query {
        commands.entity(entity).despawn();
    }
}
