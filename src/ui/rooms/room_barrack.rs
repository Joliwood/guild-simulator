use crate::{
    structs::{PlayerStats, PlayerStatsRecruitsTrigger, ResetRoomTrigger},
    ui::styles::node_container_style::node_container_style,
};
use bevy::prelude::*;

pub fn room_barrack(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    player_stats: &Res<PlayerStats>,
) {
    let imager_handler: Handle<Image> = asset_server.load("images/barrack.png");

    commands
        .spawn(NodeBundle {
            style: node_container_style(),
            ..default()
        })
        .insert(ResetRoomTrigger)
        // Image background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: imager_handler.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    ..default()
                },
                ..default()
            });
        })
        .insert(PlayerStatsRecruitsTrigger)
        // Recruits text
        .with_children(|ui_container: &mut ChildBuilder| {
            for recruit in player_stats.recruits.iter() {
                ui_container.spawn(TextBundle {
                    text: Text::from_section(
                        &recruit.class.to_string(),
                        // "one recruit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            }
        });
}
