use crate::{
    custom_components::CustomButton,
    structs::{
        general_structs::{Missions, UniqueId},
        trigger_structs::ResetRoomTrigger,
    },
    ui::{interface::gold_counter::MyAssets, styles::containers_styles::node_container_style},
};
use bevy::prelude::*;

pub fn room_command_room(
    my_assets: &Res<MyAssets>,
    commands: &mut Commands,
    missions: Res<Missions>,
) {
    // let image_handler: Handle<Image> = my_assets.load("images/command_room.png");

    commands
        .spawn(NodeBundle {
            style: node_container_style(),
            ..default()
        })
        .insert(Name::new("Command room"))
        .insert(ResetRoomTrigger)
        // Image background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: my_assets.command_room.clone().into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                z_index: ZIndex::Global(-1),
                ..default()
            });

            // Generate buttons for each mission
            for (index, mission) in missions.0.iter().enumerate() {
                if mission.recruit_send.is_none() {
                    ui_container
                        .spawn(CustomButton::Primary.bundle(my_assets))
                        .insert(UniqueId(format!("select_mission_button_{}", mission.id)))
                        .insert(mission.clone())
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::from_section(
                                    format!("Mission {}: Level {}", index + 1, mission.level),
                                    TextStyle {
                                        font: my_assets.fira_sans_bold.clone().into(),
                                        font_size: 16.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                } else {
                    ui_container
                        .spawn(CustomButton::Primary.bundle(my_assets))
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::from_section(
                                    format!("Mission {}: Level {}", index + 1, mission.level),
                                    TextStyle {
                                        font: my_assets.fira_sans_bold.clone().into(),
                                        font_size: 16.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                }
            }
        });
}
