use crate::structs::{PlayerStatsRoomTrigger, ResetRoomTrigger};
use bevy::prelude::*;

pub fn room_barrack(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    let imager_handler: Handle<Image> = asset_server.load("images/barrack.png");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
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
        });
}
