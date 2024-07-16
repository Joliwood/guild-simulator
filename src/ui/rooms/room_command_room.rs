use crate::{
    structs::{Ennemy, Mission, Missions, ResetRoomTrigger, SelectedMissionTrigger, UniqueId},
    ui::{styles::node_container_style::node_container_style, ui_constants::WOOD_COLOR},
};
use bevy::prelude::*;

pub fn room_command_room(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    missions: Res<Missions>,
) {
    let image_handler: Handle<Image> = asset_server.load("images/command_room.png");

    commands
        .spawn(NodeBundle {
            style: node_container_style(),
            ..default()
        })
        .insert(ResetRoomTrigger)
        // Image background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: image_handler.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    ..default()
                },
                ..default()
            });

            // Generate buttons for each mission
            for (index, mission) in missions.0.iter().enumerate() {
                ui_container
                    .spawn(ButtonBundle {
                        style: Style {
                            border: UiRect::all(Val::Px(5.0)),
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_radius: BorderRadius::MAX,
                        background_color: BackgroundColor(WOOD_COLOR),
                        ..default()
                    })
                    .insert((
                        SelectedMissionTrigger,
                        UniqueId("select_mission_button".to_string()),
                    ))
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
