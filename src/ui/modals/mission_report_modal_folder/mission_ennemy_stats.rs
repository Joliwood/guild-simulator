use crate::{my_assets::MyAssets, structs::general_structs::Ennemy};
use bevy::prelude::*;

pub fn mission_ennemy_stats(
    commands: &mut ChildBuilder,
    ennemy_stats: &Ennemy,
    my_assets: &Res<MyAssets>,
) {
    // ennemy Send Stats
    commands
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect {
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|stats_column| {
            // Text: "Level"
            stats_column.spawn(TextBundle {
                text: Text::from_section(
                    format!("Level: {}", ennemy_stats.level),
                    TextFont {
                        font: my_assets.fira_sans_bold.clone(),
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
                    TextFont {
                        font: my_assets.fira_sans_bold.clone(),
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
                    TextFont {
                        font: my_assets.fira_sans_bold.clone(),
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
                    TextFont {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        });
}
