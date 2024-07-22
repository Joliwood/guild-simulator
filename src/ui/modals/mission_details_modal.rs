use crate::{
    structs::{
        MissionModalVisible, ModalContentTrigger, PlayerStats, SelectedMission,
        SelectedMissionPercentOfVictoryTrigger, SelectedMissionRecruitIdTrigger, UniqueId,
    },
    styles::CustomButton,
    ui::{interface::gold_counter::MyAssets, ui_constants::WOOD_COLOR},
};
use bevy::{asset::AssetServer, prelude::*};

pub fn display_mission_modal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    modal_visible: Res<MissionModalVisible>,
    query: Query<Entity, With<ModalContentTrigger>>,
    player_stats: Res<PlayerStats>,
    selected_mission: Res<SelectedMission>,
    image_assets: Res<MyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px
    let texture_handle = asset_server.load("images/ui/buttons_atlas.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(5436, 3809),
        5,
        6,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Despawn existing modals
    if modal_visible.is_changed() && !modal_visible.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if modal_visible.is_changed() && modal_visible.0 {
        if let Some(mission) = &selected_mission.mission {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(10.0),
                        width: Val::Percent(70.0),
                        height: Val::Percent(80.0),
                        margin: UiRect::all(Val::Auto),
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Px(20.0)),
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::srgb_u8(32, 33, 36)),
                    z_index: ZIndex::Global(1),
                    ..default()
                })
                .insert(Name::new("Mission details modal"))
                .insert(ModalContentTrigger)
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            text: Text::from_section(
                                format!("Mission Details -> {:?}", mission.name),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        })
                        .insert(Name::new("Mission details modal > title"));

                    // Close button to close the modal
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    right: Val::Px(10.),
                                    top: Val::Px(10.),
                                    margin: UiRect::all(Val::Px(10.)),
                                    width: Val::Px(40.),
                                    height: Val::Px(40.),
                                    border: UiRect::all(Val::Px(3.)),
                                    ..default()
                                },
                                image: texture_handle.clone().into(),
                                border_color: BorderColor(WOOD_COLOR),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                ..default()
                            },
                            TextureAtlas {
                                index: 16,
                                layout: texture_atlas_layout.clone(),
                            },
                        ))
                        // .spawn(CustomButton::SquareIcon.bundle(&asset_server, image_assets.clone()))
                        .insert(UniqueId("close_mission_modal".to_string()))
                        // TODO WIP - Fix !
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::from_section("", TextStyle { ..default() }),
                                ..default()
                            });
                        });

                    // Main container
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                column_gap: Val::Px(10.0),
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Left container
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::Center,
                                        width: Val::Percent(30.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::WHITE),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Recruits
                                    for recruit in player_stats.recruits.iter() {
                                        parent
                                            .spawn(
                                                CustomButton::Primary
                                                    .bundle(&asset_server, image_assets.clone()),
                                            )
                                            .insert(UniqueId(format!(
                                                "assign_recruit_to_mission_{}",
                                                recruit.id
                                            )))
                                            .with_children(|button| {
                                                button.spawn(TextBundle {
                                                    text: Text::from_section(
                                                        &recruit.class.to_string(),
                                                        TextStyle {
                                                            font: asset_server
                                                                .load("fonts/FiraSans-Bold.ttf"),
                                                            font_size: 20.0,
                                                            color: Color::BLACK,
                                                        },
                                                    ),
                                                    ..default()
                                                });
                                            });
                                    }
                                });

                            // Middle container
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        width: Val::Percent(40.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::WHITE),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Middle content
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "Mission middle \n Assigned recruit :",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        ..default()
                                    });

                                    parent
                                        .spawn(TextBundle {
                                            text: Text::from_section(
                                                format!("{:?}", selected_mission.recruit_id),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(SelectedMissionRecruitIdTrigger);

                                    parent
                                        .spawn(TextBundle {
                                            text: Text::from_section(
                                                format!(
                                                    "{:?}",
                                                    selected_mission.percent_of_victory
                                                ),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(SelectedMissionPercentOfVictoryTrigger);

                                    // Button inside the middle container
                                    parent
                                        .spawn(
                                            CustomButton::Primary
                                                .bundle(&asset_server, image_assets.clone()),
                                        )
                                        .insert(UniqueId("start_mission".to_string()))
                                        .with_children(|button| {
                                            button.spawn(TextBundle {
                                                text: Text::from_section(
                                                    "Start the mission",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 20.0,
                                                        color: Color::BLACK,
                                                    },
                                                ),
                                                ..default()
                                            });
                                        });
                                });

                            // Right container
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::Center,
                                        width: Val::Percent(30.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::WHITE),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Right content
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            format!(
                                                "Ennemy for this mission -> {:?}",
                                                mission.ennemy.name
                                            ),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        ..default()
                                    });

                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            format!("Strength: {:?}", mission.ennemy.strength),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        ..default()
                                    });

                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            format!("Endurance: {:?}", mission.ennemy.endurance),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        ..default()
                                    });

                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            format!(
                                                "Intelligence: {:?}",
                                                mission.ennemy.intelligence
                                            ),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        ..default()
                                    });

                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            format!("Level: {:?}", mission.ennemy.level),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        });
                });
        }
    }
}
