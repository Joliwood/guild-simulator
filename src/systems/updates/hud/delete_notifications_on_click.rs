use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        general_structs::MissionNotificationsNumber,
        trigger_structs::{
            MissionNotificationTrigger, MissionReportModalSignButtonTrigger,
            NotificationToastTrigger,
        },
    },
};
use bevy::{asset::AssetServer, prelude::*, ui::Interaction};

pub fn delete_notifications_on_click(
    mut mission_notifications_number: ResMut<MissionNotificationsNumber>,
    mut commands: Commands,
    query: Query<Entity, With<NotificationToastTrigger>>,
    mut interaction_query_click_notification_popup: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<MissionNotificationTrigger>),
    >,
    my_assets: Res<AssetServer>,
    mut interaction_query_click_stack_of_reports: Query<
        (Entity, &Interaction),
        (
            Changed<Interaction>,
            With<MissionReportModalSignButtonTrigger>,
        ),
    >,
) {
    for (_entity, interaction) in interaction_query_click_notification_popup.iter_mut() {
        if *interaction == Interaction::Pressed {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            play_sound(&my_assets, &mut commands, SoundEnum::BookThrowDown);
            mission_notifications_number.0 = 0;
        }
    }

    for (_entity, interaction) in interaction_query_click_stack_of_reports.iter_mut() {
        if *interaction == Interaction::Pressed {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            mission_notifications_number.0 = 0;
        }
    }
}
