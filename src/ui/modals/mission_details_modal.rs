use super::{
    left_container::spawn_left_container, middle_container::spawn_middle_container,
    right_container::spawn_right_container,
};

use crate::{
    structs::{
        general_structs::{MissionModalVisible, Missions, PlayerStats, SelectedMission, UniqueId},
        trigger_structs::ModalContentTrigger,
    },
    ui::{
        interface::gold_counter::MyAssets,
        styles::containers_styles::mission_details_modal_container, ui_constants::WOOD_COLOR,
    },
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn display_mission_modal(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    modal_visible: Res<MissionModalVisible>,
    query: Query<Entity, With<ModalContentTrigger>>,
    player_stats: Res<PlayerStats>,
    selected_mission: Res<SelectedMission>,
    image_assets: Res<MyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    missions: Res<Missions>,
) {
    // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px
    // let texture_handle = my_assets.load("images/ui/buttons_atlas.png");
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
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: if modal_visible.0 {
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8))
                    } else {
                        BackgroundColor(Color::NONE)
                    },
                    ..default()
                })
                .insert(ModalContentTrigger)
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: mission_details_modal_container(),
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
                                            font: my_assets.fira_sans_bold.clone().into(),
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
                                        image: my_assets.buttons_atlas.clone().into(),
                                        border_color: BorderColor(WOOD_COLOR),
                                        border_radius: BorderRadius::all(Val::Px(10.)),
                                        ..default()
                                    },
                                    TextureAtlas {
                                        index: 16,
                                        layout: texture_atlas_layout.clone(),
                                    },
                                ))
                                // .spawn(CustomButton::SquareIcon.bundle(&my_assets, image_assets.clone()))
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
                                    spawn_left_container(parent, &my_assets, &player_stats);

                                    spawn_middle_container(
                                        parent,
                                        &my_assets,
                                        &selected_mission,
                                        missions,
                                    );

                                    spawn_right_container(parent, &my_assets, mission);
                                });
                        });
                });
        }
    }
}
