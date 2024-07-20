use crate::{
    structs::{MissionModalVisible, ModalContentTrigger, PlayerStats, SelectedMission, UniqueId},
    styles::CustomButton,
    ui::interface::gold_counter::MyAssets,
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
) {
    // Despawn existing modals
    if modal_visible.is_changed() && !modal_visible.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if modal_visible.is_changed() && modal_visible.0 {
        if let Some(mission) = &selected_mission.0 {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(70.0),
                        height: Val::Percent(80.0),
                        margin: UiRect::all(Val::Auto),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Px(20.0)),
                    background_color: BackgroundColor(Color::srgb_u8(32, 33, 36)),
                    z_index: ZIndex::Global(1),
                    ..default()
                })
                .insert(Name::new("Mission details modal"))
                .insert(ModalContentTrigger)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Mission Details -> {:?}", mission.name),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });

                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    });

                    // Close button to close the modal
                    parent
                        .spawn(CustomButton::SquareIcon.bundle(&asset_server, image_assets.clone()))
                        .insert(UniqueId("close_mission_modal".to_string()))
                        // TODO WIP - Fix !
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::from_section(
                                    "Close",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                ..default()
                            });
                        });

                    // Left side to select which recruit to assign to which mission
                    for recruit in player_stats.recruits.iter() {
                        parent
                            .spawn(
                                CustomButton::Primary.bundle(&asset_server, image_assets.clone()),
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
        }
    }
}
