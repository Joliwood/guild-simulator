use crate::{
    custom_components::CustomButton,
    structs::{general_structs::UniqueId, missions::Missions, trigger_structs::ResetRoomTrigger},
    ui::{interface::gold_counter::MyAssets, styles::containers_styles::node_container_style},
};
use bevy::prelude::*;

pub fn room_command_room(
    my_assets: &Res<MyAssets>,
    commands: &mut Commands,
    missions: Res<Missions>,
) {
    commands
        .spawn(NodeBundle {
            style: node_container_style(),
            ..default()
        })
        .insert(Name::new("Command room"))
        .insert(ResetRoomTrigger)
        // Background Image for the Command Room
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: my_assets.command_room_background.clone().into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                z_index: ZIndex::Global(-1),
                ..default()
            });
        })
        // Command Table with all child elements inside it
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(ImageBundle {
                    image: my_assets.command_table.clone().into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        width: Val::Px(1100.0),
                        height: Val::Px(600.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(0), // Ensure the table is in the background of other UI elements
                    ..default()
                })
                .with_children(|table| {
                    // Left Column
                    table
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::FlexStart,
                                width: Val::Percent(20.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|left_column| {
                            // Left column with three buttons
                            for (index, mission) in missions.0.iter().take(3).enumerate() {
                                left_column
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            width: Val::Px(100.0),
                                            height: Val::Px(50.0),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|button| {
                                        button.spawn(TextBundle {
                                            text: Text::from_section(
                                                format!("Left Button {}", index + 1),
                                                TextStyle {
                                                    font: my_assets.fira_sans_bold.clone(),
                                                    font_size: 16.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        });
                                    });
                            }
                        });

                    // Center Area (Big node)
                    table
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(60.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|center| {
                            // External function for the center area
                            spawn_center_area(center, my_assets);
                        });

                    // Right Column
                    table
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::FlexEnd,
                                width: Val::Percent(20.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|right_column| {
                            // Right column with three buttons
                            for (index, mission) in missions.0.iter().take(3).enumerate() {
                                right_column
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            width: Val::Px(100.0),
                                            height: Val::Px(50.0),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|button| {
                                        button.spawn(TextBundle {
                                            text: Text::from_section(
                                                format!("Right Button {}", index + 1),
                                                TextStyle {
                                                    font: my_assets.fira_sans_bold.clone(),
                                                    font_size: 16.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        });
                                    });
                            }
                        });
                });
        });
}

// External function for the center area (1 big child)
fn spawn_center_area(parent: &mut ChildBuilder, my_assets: &Res<MyAssets>) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                ..default()
            },
            ..default()
        })
        .with_children(|button| {
            button.spawn(TextBundle {
                text: Text::from_section(
                    "Center Area Content",
                    TextStyle {
                        font: my_assets.fira_sans_bold.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
}
