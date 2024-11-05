use crate::{my_assets::FONT_FIRA, structs::recruits::RecruitStats};
use bevy::prelude::*;

pub fn recruit_sent_stats(
    commands: &mut ChildBuilder,
    recruit_sent: &RecruitStats,
    my_assets: &Res<AssetServer>,
) {
    let recruit_strength =
        recruit_sent.strength + recruit_sent.get_additional_strength_from_items() as u16;

    let recruit_endurance: u16 =
        recruit_sent.endurance + recruit_sent.get_additional_endurance_from_items() as u16;

    let recruit_intelligence: u16 =
        recruit_sent.intelligence + recruit_sent.get_additional_intelligence_from_items() as u16;

    // Recruit Send Stats
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
                Text::new(format!("Level: {}", recruit_sent.level)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Text: "Strength"
            stats_column.spawn((
                Text::new(format!("Strength: {}", recruit_strength)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Text: "Endurance"
            stats_column.spawn((
                Text::new(format!("Endurance: {}", recruit_endurance)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Text: "Intelligence"
            stats_column.spawn((
                Text::new(format!("Intelligence: {}", recruit_intelligence)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}
