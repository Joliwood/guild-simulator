#![allow(unused_imports)]
use super::{
    daily_event_documents::daily_event_documents,
    mission_report_documents::mission_report_documents,
};
use crate::structs::{
    daily_events_folder::daily_events::DailyEvents, general_structs::MissionReportsModalVisible,
    missions::MissionReports, trigger_structs::ResetRoomTrigger,
};
use bevy::prelude::*;
// use vleue_kinetoscope::*;

pub fn room_office(
    my_assets: &Res<AssetServer>,
    commands: &mut Commands,
    mission_reports: &Res<MissionReports>,
    _mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    daily_events: &Res<DailyEvents>,
) {
    // let animated_image = my_assets.load("images/rooms/office/animated_background1.webp");
    commands
        .spawn((
            // AnimatedImageController::play(animated_image),
            // ImageNode {
            //     // image: my_assets.load("images/rooms/office/animated_background1.webp"),
            //     image: my_assets.load("images/rooms/office/desk.png"),
            //     ..default()
            // },
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                ..default()
            },
            // GlobalZIndex(-1),
        ))
        .insert(Name::new("Office room"))
        .insert(ResetRoomTrigger)
        // ImageNode background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn((
                    ImageNode {
                        image: my_assets.load("images/rooms/office/office_room_background.png"),
                        // image: my_assets.load("images/rooms/office/office_v2.png"),
                        ..default()
                    },
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    GlobalZIndex(-1),
                ))
                // Nested image background node
                .with_children(|desk_container: &mut ChildBuilder| {
                    desk_container
                        .spawn((
                            ImageNode {
                                image: my_assets.load("images/rooms/office/desk.png"),
                                ..default()
                            },
                            Node {
                                display: Display::Flex,
                                align_self: AlignSelf::Center,
                                width: Val::Percent(90.0),
                                height: Val::Percent(80.0),
                                ..default()
                            },
                        ))
                        // Adding child nodes with different positions
                        .with_children(|elements_on_desk: &mut ChildBuilder| {
                            mission_report_documents(my_assets, elements_on_desk, mission_reports);
                            // recap_guild_scroll(&asset_server, elements_on_desk);
                            // talents_on_desk(&asset_server, elements_on_desk);
                            daily_event_documents(my_assets, elements_on_desk, daily_events);
                        });
                });
        });
}
