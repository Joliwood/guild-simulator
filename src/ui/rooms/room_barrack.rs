use crate::{
    structs::{PlayerStats, PlayerStatsRecruitsTrigger, ResetRoomTrigger, UniqueId},
    ui::{styles::node_container_style::node_container_style, ui_constants::WOOD_COLOR},
};
use bevy::prelude::*;

pub fn room_barrack(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    player_stats: &Res<PlayerStats>,
) {
    let image_handle: Handle<Image> = asset_server.load("images/barrack.png");

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                margin: UiRect::all(Val::Auto),
                height: Val::Percent(90.0),
                ..node_container_style()
            },
            ..default()
        })
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

                    // Left side: list of recruit buttons
                    for recruit in player_stats.recruits.iter() {
                        left_container
                            .spawn(ButtonBundle {
                                style: Style {
                                    border: UiRect::all(Val::Px(5.0)),
                                    width: Val::Px(150.0),
                                    height: Val::Px(65.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_radius: BorderRadius::MAX,
                                background_color: BackgroundColor(WOOD_COLOR),
                                ..default()
                            })
                            .insert(UniqueId("recruit_buttons".to_string()))
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
                .with_children(|right_container| {
                    right_container.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Percent(80.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
                        ..default()
                    });
                })
                // All infos of the selected button
                ;
        });
}
