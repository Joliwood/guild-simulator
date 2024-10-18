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

use super::{
    loots_and_start::loots_and_start, mission_recap::mission_recap, recruit_recap::recruit_recap,
};

#[allow(clippy::too_many_arguments)]
pub fn mission_order_modal(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    mission_modal_visibility: Res<MissionModalVisible>,
    query: Query<Entity, With<MissionModalContentTrigger>>,
    player_stats: Res<PlayerStats>,
    selected_mission: Res<SelectedMission>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    _missions: Res<Missions>,
    selected_recruit_for_mission: Res<SelectedRecruitForMission>,
) {
    let buttons_layout = TextureAtlasLayout::from_grid(
        UVec2::new(5436, 3809),
        5,
        6,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let buttons_texture_atlas_layout = texture_atlas_layouts.add(buttons_layout);

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
                    background_color: BackgroundColor(WOOD_COLOR),
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
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        })
                        .insert(Name::new("Mission details modal > title"));

                    // Main contents / loots
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
                            // Main contents : mission image and description, percent of win, recruit info, and button
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(80.),
                                        row_gap: Val::Px(20.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    mission_recap(
                                        parent,
                                        &my_assets,
                                        mission,
                                        &mut texture_atlas_layouts,
                                    );

                                    if selected_mission.percent_of_victory.is_some() {
                                        // Percent of Win (centered)
                                        parent.spawn(TextBundle {
                                            text: Text::from_section(
                                                format!(
                                                    "{}%",
                                                    selected_mission.percent_of_victory.unwrap()
                                                ),
                                                TextStyle {
                                                    font: my_assets.fira_sans_bold.clone(),
                                                    font_size: 18.0,
                                                    color: Color::BLACK,
                                                },
                                            ),
                                            style: Style {
                                                align_self: AlignSelf::Center,
                                                justify_self: JustifySelf::Center,
                                                ..default()
                                            },
                                            ..default()
                                        });
                                    }

                                    recruit_recap(
                                        parent,
                                        selected_recruit_for_mission,
                                        &my_assets,
                                        &mut texture_atlas_layouts,
                                        &player_stats,
                                    );
                                });

                            loots_and_start(parent, &my_assets);
                        });
                });
        }
    }
}
