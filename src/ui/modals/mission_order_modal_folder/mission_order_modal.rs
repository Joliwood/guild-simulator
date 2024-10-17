use crate::{
    structs::{
        general_structs::{MissionModalVisible, UniqueId},
        missions::{Missions, SelectedMission},
        player_stats::PlayerStats,
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
    if mission_modal_visibility.is_changed() && !mission_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if mission_modal_visibility.is_changed() && mission_modal_visibility.0 {
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

                    // Close button to close the modal
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    right: Val::Px(10.),
                                    top: Val::Px(10.),
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
                            // spawn_left_container(parent, &my_assets, &player_stats);

                            // spawn_middle_container(
                            //     parent,
                            //     &my_assets,
                            //     &selected_mission,
                            //     missions,
                            // );

                            // spawn_right_container(parent, &my_assets, mission);
                        });
                });
        }
    }
}
