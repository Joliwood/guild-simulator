use crate::{
    structs::{
        general_structs::{Missions, UniqueId},
        trigger_structs::ResetRoomTrigger,
    },
    styles::CustomButton,
    ui::{interface::gold_counter::MyAssets, styles::node_container_style::node_container_style},
};
use bevy::prelude::*;

pub fn room_command_room(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    missions: Res<Missions>,
    image_assets: &Res<MyAssets>,
) {
    let image_handler: Handle<Image> = asset_server.load("images/command_room.png");

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
                image: image_handler.into(),
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
                ui_container
                    .spawn(CustomButton::Primary.bundle(&asset_server, image_assets))
                    .insert(UniqueId(format!("select_mission_button_{}", mission.id)))
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text::from_section(
                                format!("Mission {}: Level {}", index + 1, mission.level),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}
