use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        general_structs::MissionNotificationsNumber,
        trigger_structs::{MissionNotificationTrigger, NotificationToastTrigger},
    },
};
use bevy::{
    asset::AssetServer,
    prelude::{Changed, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut, With},
    ui::Interaction,
};

pub fn delete_notifications_on_click(
    mut mission_notifications_number: ResMut<MissionNotificationsNumber>,
    mut commands: Commands,
    query: Query<Entity, With<NotificationToastTrigger>>,
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<MissionNotificationTrigger>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (_entity, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            play_sound(&asset_server, &mut commands, SoundEnum::BookThrowDown);
            mission_notifications_number.0 = 0;
        }
    }
}
