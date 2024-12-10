#![allow(unused_imports)]
use super::{
    daily_event_documents::daily_event_documents,
    mission_report_documents::mission_report_documents,
};
use crate::enums::RoomEnum;
use crate::structs::{
    daily_events_folder::daily_events::DailyEvents, general_structs::MissionReportsModalVisible,
    missions::MissionReports, trigger_structs::ResetRoomTrigger,
};
use crate::{my_assets, RoomTag};
use bevy::{prelude::*, text::cosmic_text::Align};
use bevy_light_2d::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn room_office(
    my_assets: &Res<AssetServer>,
    commands: &mut Commands,
    mission_reports: &Res<MissionReports>,
    _mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    daily_events: &Res<DailyEvents>,
) {
    commands
        .spawn((
            ImageNode {
                image: my_assets.load("images/rooms/office/office_v4.png"),
                ..default()
            },
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                ..default()
            },
            GlobalZIndex(-1),
            RoomTag(RoomEnum::Office),
        ))
        .insert(Name::new("Office room"))
        .insert(ResetRoomTrigger)
        // Nested image background node
        .with_children(|desk_container: &mut ChildBuilder| {
            desk_container
                .spawn((Node {
                    display: Display::Flex,
                    align_self: AlignSelf::Center,
                    width: Val::Percent(90.0),
                    height: Val::Percent(80.0),
                    ..default()
                },))
                // Adding child nodes with different positions
                .with_children(|elements_on_desk: &mut ChildBuilder| {
                    mission_report_documents(my_assets, elements_on_desk, mission_reports);
                    // recap_guild_scroll(&asset_server, elements_on_desk);
                    // talents_on_desk(&asset_server, elements_on_desk);
                    daily_event_documents(my_assets, elements_on_desk, daily_events);
                });
        });
}
