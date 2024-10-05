use super::{
    mission_report_documents::mission_report_documents, recap_guild_scroll::recap_guild_scroll,
    talents_on_desk::talents_on_desk,
};
use crate::{
    structs::trigger_structs::ResetRoomTrigger,
    ui::{interface::gold_counter::MyAssets, styles::containers_styles::node_container_style},
};
use bevy::prelude::*;

pub fn room_office(my_assets: &Res<MyAssets>, commands: &mut Commands) {
    commands
        .spawn(NodeBundle {
            style: node_container_style(),
            ..default()
        })
        .insert(Name::new("Office room"))
        .insert(ResetRoomTrigger)
        // Image background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(ImageBundle {
                    image: my_assets.office.clone().into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    z_index: ZIndex::Global(-1),
                    ..default()
                })
                // Nested image background node
                .with_children(|desk_container: &mut ChildBuilder| {
                    desk_container
                        .spawn(ImageBundle {
                            image: my_assets.desk.clone().into(),
                            style: Style {
                                display: Display::Flex,
                                align_self: AlignSelf::Center,
                                width: Val::Percent(90.0),
                                height: Val::Percent(80.0),
                                ..default()
                            },
                            ..default()
                        })
                        // Adding child nodes with different positions
                        .with_children(|elements_on_desk: &mut ChildBuilder| {
                            mission_report_documents(&my_assets, elements_on_desk);
                            recap_guild_scroll(&my_assets, elements_on_desk);
                            talents_on_desk(&my_assets, elements_on_desk);
                        });
                });
        });
}
