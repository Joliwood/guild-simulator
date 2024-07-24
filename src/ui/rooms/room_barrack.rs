use crate::{
    structs::{
        PlayerStats, PlayerStatsRecruitsTrigger, ResetRoomTrigger, SelectedRecruit,
        SelectedRecruitTrigger, UniqueId,
    },
    styles::CustomButton,
    ui::{interface::gold_counter::MyAssets, styles::node_container_style::node_container_style},
};
use bevy::prelude::*;

pub fn room_barrack(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    player_stats: &Res<PlayerStats>,
    selected_recruit: &Res<SelectedRecruit>,
    image_assets: MyAssets,
) {
    let image_handle: Handle<Image> = asset_server.load("images/barrack.png");
    info!("Selected recruit: {:?}", selected_recruit.0);

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                margin: UiRect::all(Val::Auto),
                height: Val::Percent(100.0),
                ..node_container_style()
            },
            z_index: ZIndex::Global(-1),
            ..default()
        })
        .insert(Name::new("Room barrack"))
        .insert(ResetRoomTrigger)
        .insert(PlayerStatsRecruitsTrigger)
        // Left container
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(40.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Barrack > left container"))
                .with_children(|left_container| {
                    // Background image
                    left_container.spawn(ImageBundle {
                        image: image_handle.into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    });

                    // Barrack room > left container > recruit buttons
                    for recruit in player_stats.recruits.iter() {
                        left_container
                            .spawn(
                                CustomButton::Primary.bundle(&asset_server, image_assets.clone()),
                            )
                            .insert((
                                UniqueId(format!("recruit_button_{}", recruit.id)),
                                SelectedRecruitTrigger,
                            )) // Use recruit.id here
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    text: Text::from_section(
                                        &recruit.class.to_string(),
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });

            // Right container: recruit info
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Barrack > right container"))
                .with_children(|right_container| {
                    right_container.spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(80.0),
                            height: Val::Percent(80.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
                        border_radius: BorderRadius::all(Val::Px(20.0)),
                        ..default()
                    });
                })
                .insert(SelectedRecruitTrigger)
                .with_children(|right_container| {
                    if let Some(recruit) = &selected_recruit.0 {
                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("ID: {:?}", recruit.id),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });

                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Class: {:?}", recruit.class),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });

                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Endurance: {:?}", recruit.endurance),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });

                        right_container
                            .spawn(TextBundle {
                                text: Text::from_section(
                                    format!("Experience: {:?}", recruit.experience),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                style: Style {
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(SelectedRecruitTrigger);

                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Intelligence: {:?}", recruit.intelligence),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });

                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Level: {:?}", recruit.level),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });

                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Max Experience: {:?}", recruit.max_experience),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });

                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Strength: {:?}", recruit.strength),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });
                    } else {
                        right_container.spawn(TextBundle {
                            text: Text::from_section(
                                "No recruit selected".to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        });
                    }
                });
        });
}
