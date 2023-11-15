use crate::BUTTON;
use bevy::prelude::*;

mod back;
mod to_main_menu;
pub use back::back_game;
pub use to_main_menu::to_main;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct BackGame;

#[derive(Component)]
pub struct Exit;

pub fn pause_menu(mut commands: Commands) {
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
            PauseMenu,
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
                    PauseMenu,
                    BackGame,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Back",
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
                        PauseMenu,
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
                    PauseMenu,
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
                        PauseMenu,
                    ));
                });
        });
}

pub fn pause_despawn(mut commands: Commands, query: Query<(Entity, With<PauseMenu>)>) {
    for (entity, _) in &query {
        commands.entity(entity).despawn();
    }
}
