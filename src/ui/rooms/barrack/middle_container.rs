use crate::structs::{general_structs::SelectedRecruit, trigger_structs::SelectedRecruitTrigger};
use bevy::prelude::*;

pub fn spawn_middle_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    selected_recruit: &Res<SelectedRecruit>,
) {
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
}
