use crate::{my_assets::FONT_FIRA, structs::general_structs::Ennemy};
use bevy::prelude::*;

pub fn mission_ennemy_stats(
    commands: &mut ChildBuilder,
    ennemy_stats: &Ennemy,
    my_assets: &Res<AssetServer>,
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
            stats_column.spawn((
                Text::new(format!("Level: {}", ennemy_stats.level)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Text: "Strength"
            stats_column.spawn((
                Text::new(format!("Strength: {}", ennemy_stats.strength)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Text: "Endurance"
            stats_column.spawn((
                Text::new(format!("Endurance: {}", ennemy_stats.endurance)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Text: "Intelligence"
            stats_column.spawn((
                Text::new(format!("Intelligence: {}", ennemy_stats.intelligence)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}
