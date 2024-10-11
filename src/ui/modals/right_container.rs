use crate::{structs::general_structs::Mission, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

/// Spawns the left container, displaying the player's recruits.
pub fn spawn_right_container(
    parent: &mut ChildBuilder,
    my_assets: &Res<MyAssets>,
    mission: &Mission,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                width: Val::Percent(30.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(Color::WHITE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Ennemy for this mission -> {:?}", mission.ennemy.name),
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone().into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Strength: {:?}", mission.ennemy.strength),
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone().into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Endurance: {:?}", mission.ennemy.endurance),
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone().into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Intelligence: {:?}", mission.ennemy.intelligence),
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone().into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Level: {:?}", mission.ennemy.level),
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone().into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
        });
}
