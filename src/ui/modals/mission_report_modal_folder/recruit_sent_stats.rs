use crate::{my_assets::FONT_FIRA, structs::recruits::RecruitStats};
use bevy::prelude::*;

pub fn recruit_sent_stats(
    commands: &mut ChildBuilder,
    recruit_sent: &RecruitStats,
    my_assets: &Res<AssetServer>,
) {
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
            stats_column.spawn((
                Text::new(format!("{}: {}", t!("level"), recruit_sent.level)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            stats_column.spawn((
                Text::new(format!("ATT: {}", recruit_sent.attack)),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}
