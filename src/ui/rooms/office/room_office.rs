#![allow(unused_imports)]
use super::{mission_report_documents::mission_report_documents, set_of_keys::set_of_keys};
use crate::{
    my_assets::MyAssets,
    structs::{
        general_structs::MissionReportsModalVisible, missions::MissionReports,
        trigger_structs::ResetRoomTrigger,
    },
    ui::styles::containers_styles::node_container_style,
};
use bevy::prelude::*;

pub fn room_office(
    my_assets: &Res<MyAssets>,
    commands: &mut Commands,
    mission_reports: &Res<MissionReports>,
    _mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
) {
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
                    image: my_assets.office_background.clone().into(),
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
                            mission_report_documents(my_assets, elements_on_desk, mission_reports);
                            // TODO - Futur features, not necessary for V0
                            // recap_guild_scroll(&asset_server, elements_on_desk);
                            // talents_on_desk(&asset_server, elements_on_desk);
                            set_of_keys(my_assets, elements_on_desk, mission_reports);
                        });
                });
        });
}
