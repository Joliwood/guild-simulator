use crate::structs::recruits::RecruitStats;
use bevy::prelude::*;

pub fn recruit_sent_stats(
    commands: &mut ChildBuilder,
    recruit_sent: &RecruitStats,
    asset_server: &Res<AssetServer>,
) {
    let recruit_strength =
        recruit_sent.strength + recruit_sent.get_additional_strength_from_items() as u16;

    let recruit_endurance: u16 =
        recruit_sent.endurance + recruit_sent.get_additional_endurance_from_items() as u16;

    let recruit_intelligence: u16 =
        recruit_sent.intelligence + recruit_sent.get_additional_intelligence_from_items() as u16;

    // Recruit Send Stats
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
                    format!("Level: {}", recruit_sent.level),
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
                    format!("Strength: {}", recruit_strength),
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
                    format!("Endurance: {}", recruit_endurance),
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
                    format!("Intelligence: {}", recruit_intelligence),
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
