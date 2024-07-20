use crate::{
    structs::ResetRoomTrigger,
    ui::{buttons::gold_button::gold_button, styles::node_container_style::node_container_style},
};
use bevy::prelude::*;

pub fn room_office(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let imager_handler: Handle<Image> = asset_server.load("images/office.png");

    commands
        .spawn(NodeBundle {
            style: node_container_style(),
            ..default()
        })
        .insert(Name::new("Office room"))
        .insert(ResetRoomTrigger)
        // Image background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: imager_handler.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(1.0),
                    height: Val::Percent(1.0),
                    display: Display::Flex,
                    ..default()
                },
                z_index: ZIndex::Global(-1),
                ..default()
            });
        })
        // Menu button node
        .with_children(|settings_button: &mut ChildBuilder| {
            gold_button(asset_server, settings_button, texture_atlas_layouts);
        });
}
