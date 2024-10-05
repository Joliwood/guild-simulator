use super::{
    mission_report_documents::mission_report_documents, recap_guild_scroll::recap_guild_scroll,
    set_of_keys::set_of_keys, talents_on_desk::talents_on_desk,
};
use crate::{
    structs::{
        general_structs::MissionReportsModalVisible, missions::MissionReports,
        trigger_structs::ResetRoomTrigger,
    },
    ui::{
        modals::mission_report_modal::mission_report_modal,
        styles::containers_styles::node_container_style,
    },
};
use bevy::prelude::*;

pub fn room_office(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    mission_reports: ResMut<MissionReports>,
    mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
) {
    let background_handle: Handle<Image> =
        asset_server.load("images/rooms/office/office_room_background.png");
    let desk_image_handler: Handle<Image> = asset_server.load("images/rooms/office/desk.png");

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
                            mission_report_documents(
                                &asset_server,
                                elements_on_desk,
                                mission_reports,
                            );
                            recap_guild_scroll(&asset_server, elements_on_desk);
                            talents_on_desk(&asset_server, elements_on_desk);
                            set_of_keys(asset_server, elements_on_desk);
                            // if mission_reports_modal_visibility.0 == true {
                            //     mission_report_modal(
                            //         elements_on_desk,
                            //         asset_server,
                            //         // mission_reports_modal_visibility,
                            //     );
                            // }
                        });
                });
        });
}
