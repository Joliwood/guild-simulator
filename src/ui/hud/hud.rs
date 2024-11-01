use crate::{enums::ColorPaletteEnum, my_assets::MyAssets, structs::player_stats::PlayerStats};
use bevy::prelude::*;

pub fn hud(my_assets: Res<MyAssets>, mut commands: Commands, player_stats: Res<PlayerStats>) {
    commands
        // Main Container
        .spawn(ImageBundle {
            image: my_assets.hud.clone().into(),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                justify_content: JustifyContent::SpaceBetween,
                display: Display::Flex,
                ..default()
            },
            z_index: ZIndex::Global(3),
            ..default()
        })
        .insert(Name::new("HUD"))
        // Left Container
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Px(205.0),
                        height: Val::Px(25.0),
                        ..default()
                    },
                    background_color: BackgroundColor(ColorPaletteEnum::Brown.as_color()),
                    ..default()
                })
                .insert(Name::new("Left Container"))
                .with_children(|left_container| {
                    // Adding 3 text elements
                    left_container.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Gold: {}", player_stats.golds),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                    left_container.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Guild Level: {}", player_stats.guild_level),
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                    left_container.spawn(TextBundle {
                        text: Text::from_section(
                            "Status: Active",
                            TextStyle {
                                font: my_assets.fira_sans_bold.clone(),
                                font_size: 14.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                });
            // Middle Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        height: Val::Px(35.0),
                        width: Val::Px(165.0),
                        margin: UiRect {
                            right: Val::Px(2.),
                            ..default()
                        },
                        column_gap: Val::Px(7.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Middle Container"))
                .with_children(|middle_container| {
                    // Adding 3 button elements
                    for i in 1..=3 {
                        middle_container
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Percent(100.),
                                    ..default()
                                },
                                background_color: BackgroundColor(
                                    ColorPaletteEnum::Brown.as_color(),
                                ),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    text: Text::from_section(
                                        i.to_string(),
                                        TextStyle {
                                            font: my_assets.fira_sans_bold.clone(),
                                            font_size: 14.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });
            // Right Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Px(208.0),
                        height: Val::Px(25.0),
                        ..default()
                    },
                    background_color: BackgroundColor(ColorPaletteEnum::Brown.as_color()),
                    ..default()
                })
                .insert(Name::new("Right Container"))
                .with_children(|right_container| {
                    // Adding 3 fake text elements
                    for i in 1..=3 {
                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Text {}", i),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 14.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        });
                    }
                });
        });
}
