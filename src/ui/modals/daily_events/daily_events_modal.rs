use super::discussion_event_document::discussion_event_document;
use crate::{
    enums::ColorPaletteEnum,
    my_assets::MyAssets,
    structs::{
        daily_events::{DailyDiscussionEnum, DailyEvent, DailyEventTypeEnum, DailyEvents},
        general_structs::DailyEventsModalVisible,
        missions::Missions,
        player_stats::PlayerStats,
        trigger_structs::{DailyEventsModalContentTrigger, MissionReportModalSignButtonTrigger},
    },
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
// Show daily events documents on desk
pub fn daily_events_modal(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    query: Query<Entity, With<DailyEventsModalContentTrigger>>,
    daily_events_modal_visibility: Res<DailyEventsModalVisible>,
    daily_events: Res<DailyEvents>,
    missions: Res<Missions>,
    player_stats: ResMut<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Despawn existing modals if visibility is set to false
    if daily_events_modal_visibility.is_changed() && !daily_events_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let mission_reports_len = daily_events.0.len();

    // Spawn the mission report modal if visibility is true and there are mission reports
    if daily_events_modal_visibility.is_changed()
        && daily_events_modal_visibility.0
        && mission_reports_len > 0
    {
        // let container_image: Handle<Image> =
        //     asset_server.load("images/rooms/barrack/inventory_container.png");

        let last_daily_event: &DailyEvent = match daily_events.get_last_daily_event() {
            Some(report) => report,
            None => return,
        };

        match last_daily_event.daily_event_type {
            DailyEventTypeEnum::Discussion(_) => {
                discussion_event_document(
                    &mut commands,
                    &my_assets,
                    &last_daily_event,
                    // &mut texture_atlas_layouts,
                );
            }
            DailyEventTypeEnum::SpontaneousApplication(_) => {
                // let daily_application: &DailyApplicationEnum = match last_daily_event.daily_event {
                //     DailyEvent::Application(application) => application,
                //     _ => return,
                // };
            }
        }
    }
}
