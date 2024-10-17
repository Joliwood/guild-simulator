use crate::{
    enums::ColorPaletteEnum,
    structs::{
        general_structs::{MissionModalVisible, UniqueId},
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
        recruits::SelectedRecruitForMission,
        trigger_structs::MissionModalContentTrigger,
    },
    ui::{interface::gold_counter::MyAssets, ui_constants::WOOD_COLOR},
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn mission_order_modal(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    mission_modal_visibility: Res<MissionModalVisible>,
    query: Query<Entity, With<MissionModalContentTrigger>>,
    player_stats: Res<PlayerStats>,
    selected_mission: Res<SelectedMission>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    missions: Res<Missions>,
    selected_recruit_for_mission: Res<SelectedRecruitForMission>,
) {
    // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px
    // let texture_handle = my_assets.load("images/ui/buttons_atlas.png");
    let buttons_layout = TextureAtlasLayout::from_grid(
        UVec2::new(5436, 3809),
        5,
        6,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let buttons_texture_atlas_layout = texture_atlas_layouts.add(buttons_layout);

    let ennemy_layout = TextureAtlasLayout::from_grid(
        UVec2::new(1200, 200),
        6,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let ennemy_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(ennemy_layout);

    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(800, 200),
        5,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    // Despawn existing modals
    if mission_modal_visibility.is_changed() && !mission_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Despawn and retrigger if the recruit is changed
    if (selected_recruit_for_mission.is_changed() || mission_modal_visibility.is_changed())
        && mission_modal_visibility.0
        && selected_mission.mission.is_some()
    {
        // Despawn existing modals when retriggering
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        if let Some(mission) = &selected_mission.mission {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(10.0),
                        width: Val::Px(570.0),
                        height: Val::Px(470.0),
                        margin: UiRect::all(Val::Auto),
                        padding: UiRect::all(Val::Px(10.0)),
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
                .insert(MissionModalContentTrigger)
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    right: Val::Px(5.),
                                    top: Val::Px(5.),
                                    width: Val::Px(30.),
                                    height: Val::Px(30.),
                                    border: UiRect::all(Val::Px(3.)),
                                    ..default()
                                },
                                image: my_assets.buttons_atlas.clone().into(),
                                border_color: BorderColor(WOOD_COLOR),
                                border_radius: BorderRadius::all(Val::Px(10.)),
                                ..default()
                            },
                            TextureAtlas {
                                index: 16,
                                layout: buttons_texture_atlas_layout.clone(),
                            },
                        ))
                        .insert(UniqueId("close_mission_modal".to_string()));

                    // Title
                    parent
                        .spawn(TextBundle {
                            text: Text::from_section(
                                mission.name.to_string(),
                                TextStyle {
                                    font: my_assets.fira_sans_bold.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        })
                        .insert(Name::new("Mission details modal > title"));

                    // Recruits (Image / Name / Stats / Node Container)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Main container: mission image and description, percent of win, recruit info, loots, and button
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        row_gap: Val::Px(20.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(
                                        ColorPaletteEnum::Info.as_color(),
                                    ),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Mission Image and Description (A / B)
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Column,
                                                justify_content: JustifyContent::SpaceBetween,
                                                width: Val::Percent(50.0),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(
                                                ColorPaletteEnum::Brown.as_color(),
                                            ),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            // Mission image
                                            parent.spawn((
                                                ImageBundle {
                                                    image: my_assets
                                                        .ennemy_image_handle
                                                        .clone()
                                                        .into(),
                                                    style: Style {
                                                        width: Val::Px(100.0),
                                                        height: Val::Px(100.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                TextureAtlas {
                                                    index: mission.ennemy.image_atlas_index.into(),
                                                    layout: ennemy_texture_atlas_layout.clone(),
                                                },
                                            ));

                                            // Mission description
                                            parent.spawn(TextBundle {
                                                text: Text::from_section(
                                                    mission.description.clone(),
                                                    TextStyle {
                                                        font: my_assets.fira_sans_bold.clone(),
                                                        font_size: 16.0,
                                                        color: Color::BLACK,
                                                    },
                                                ),
                                                ..default()
                                            });
                                        });

                                    // Percent of Win (centered)
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            format!(
                                                "{}%",
                                                mission.percent_of_victory.unwrap_or_default()
                                            ),
                                            TextStyle {
                                                font: my_assets.fira_sans_bold.clone(),
                                                font_size: 18.0,
                                                color: Color::BLACK,
                                            },
                                        ),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.0)),
                                            align_self: AlignSelf::Center,
                                            width: Val::Percent(10.0),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(
                                            ColorPaletteEnum::Brown.as_color(),
                                        ),
                                        ..default()
                                    });

                                    // Recruits (Image / Name / Stats / Node Container)
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(40.0),
                                                flex_direction: FlexDirection::Column,
                                                justify_content: JustifyContent::SpaceBetween,
                                                column_gap: Val::Px(10.0),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(
                                                ColorPaletteEnum::Primary.as_color(),
                                            ),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            if selected_recruit_for_mission.0.is_none() {
                                                parent.spawn(TextBundle {
                                                    text: Text::from_section(
                                                        "No recruit selected",
                                                        TextStyle {
                                                            font: my_assets.fira_sans_bold.clone(),
                                                            font_size: 16.0,
                                                            color: Color::BLACK,
                                                        },
                                                    ),
                                                    ..default()
                                                });
                                                return;
                                            } else {
                                                // Recruit image
                                                parent.spawn((
                                                    ImageBundle {
                                                        image: my_assets
                                                            .recruit_picture_atlas
                                                            .clone()
                                                            .into(),
                                                        style: Style {
                                                            width: Val::Px(50.0),
                                                            height: Val::Px(50.0),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    TextureAtlas {
                                                        index: selected_recruit_for_mission
                                                            .0
                                                            .clone()
                                                            .unwrap()
                                                            .image_atlas_index
                                                            .into(),
                                                        layout: recruit_texture_atlas_layout
                                                            .clone(),
                                                    },
                                                ));
                                            }

                                            // Recruit name
                                            parent.spawn(TextBundle {
                                                text: Text::from_section(
                                                    selected_recruit_for_mission
                                                        .0
                                                        .clone()
                                                        .unwrap()
                                                        .name,
                                                    TextStyle {
                                                        font: my_assets.fira_sans_bold.clone(),
                                                        font_size: 14.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });

                                            // Recruit stats (just an example)
                                            parent.spawn(TextBundle {
                                                text: Text::from_section(
                                                    format!(
                                                        "Stats: {}",
                                                        selected_recruit_for_mission
                                                            .0
                                                            .clone()
                                                            .unwrap()
                                                            .level
                                                    ),
                                                    TextStyle {
                                                        font: my_assets.fira_sans_bold.clone(),
                                                        font_size: 12.0,
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });
                                        });
                                });

                            // Loots (Text / Loot Icons)
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        column_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Loots in text
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "Loots: gold, sword",
                                            TextStyle {
                                                font: my_assets.fira_sans_bold.clone(),
                                                font_size: 14.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        ..default()
                                    });

                                    //     // Loot icons
                                    //     parent.spawn(ImageBundle {
                                    //         image: UiImage(my_assets.loot_image.clone()),
                                    //         style: Style {
                                    //             width: Val::Px(40.0),
                                    //             height: Val::Px(40.0),
                                    //             ..default()
                                    //         },
                                    //         ..default()
                                    //     });
                                    // });

                                    // Sign the mission button
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                margin: UiRect::all(Val::Px(10.0)),
                                                width: Val::Percent(20.0),
                                                height: Val::Px(50.0),
                                                ..default()
                                            },
                                            image: my_assets._play.clone().into(),
                                            ..default()
                                        })
                                        .insert(UniqueId("sign_mission_order".to_string()));
                                });
                        });
                });
        }
    }
}
