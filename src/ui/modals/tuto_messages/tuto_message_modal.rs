use crate::{
    content::daily_events::{
        discussions::get_daily_discussion, spontaneous_applications::get_spontaneous_application,
    },
    structs::{
        daily_events_folder::daily_events::{DailyEvent, DailyEventTypeEnum, DailyEvents},
        general_structs::TutoMessagesModalVisible,
        player_stats::{PlayerStats, TutoMessage, TutoMessages},
        trigger_structs::SelectAnswerTrigger,
    },
};
use bevy::prelude::*;

pub fn tuto_message_modal(
    mut commands: Commands,
    my_assets: Res<AssetServer>,
    query: Query<Entity, With<SelectAnswerTrigger>>,
    tuto_messages_modal_visibility: Res<TutoMessagesModalVisible>,
    tuto_messages: Res<TutoMessages>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Despawn existing modals if visibility is set to false
    if tuto_messages_modal_visibility.is_changed() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let tuto_messages_len = tuto_messages.0.len();

    // Spawn the mission report modal if visibility is true and there are mission reports
    if tuto_messages_modal_visibility.is_changed()
        && tuto_messages_modal_visibility.0
        && tuto_messages_len > 0
    {
        let last_tuto_message: &TutoMessage = match tuto_messages.get_last_tuto_message() {
            Some(tuto_messages) => tuto_messages,
            None => return,
        };

        // discussion_event_doc(
        //     &mut commands,
        //     &my_assets,
        //     get_daily_discussion(discussion),
        //     texture_atlas_layouts,
        // );
    }
}
