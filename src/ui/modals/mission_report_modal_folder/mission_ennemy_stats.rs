use crate::structs::general_structs::Ennemy;
use bevy::prelude::*;

pub fn mission_ennemy_stats(
    commands: &mut ChildBuilder,
    ennemy_stats: &Ennemy,
    asset_server: &Res<AssetServer>,
) {
    // ennemy Send Stats
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect {
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|stats_column| {
            // Text: "Level"
            stats_column.spawn(TextBundle {
                text: Text::from_section(
                    format!("Level: {}", ennemy_stats.level),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });

            // Text: "Strength"
            stats_column.spawn(TextBundle {
                text: Text::from_section(
                    format!("Strength: {}", ennemy_stats.strength),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });

            // Text: "Endurance"
            stats_column.spawn(TextBundle {
                text: Text::from_section(
                    format!("Endurance: {}", ennemy_stats.endurance),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });

            // Text: "Intelligence"
            stats_column.spawn(TextBundle {
                text: Text::from_section(
                    format!("Intelligence: {}", ennemy_stats.intelligence),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        });
}
