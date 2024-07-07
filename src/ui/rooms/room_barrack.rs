use crate::{structs::ResetRoomTrigger, ui::styles::node_container_style::node_container_style};
use bevy::prelude::*;

pub fn room_barrack(asset_server: &Res<AssetServer>, commands: &mut Commands) {
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
        });
}
