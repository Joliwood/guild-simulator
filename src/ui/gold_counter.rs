use crate::structs::{GoldCountText, PlayerStats};
use bevy::prelude::*;

pub fn gold_counter(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // The position absolute make the gold counter visible (z-index)
                display: Display::Flex,
                row_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                ..default()
            },
            ..default()
        })
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(ImageBundle {
                    image: asset_server.load("images/gold.png").into(),
                    style: Style {
                        // The position absolute make the gold counter visible (z-index)
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        // align_self: AlignSelf::Center,
                        // justify_self: JustifySelf::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(GoldCountText);
        })
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(TextBundle {
                    text: Text::from_section(
                        format! {"{gold_counter}", gold_counter = player_stats.golds},
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(GoldCountText);
        });
}
