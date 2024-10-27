use super::discussion_event_doc::discussion_event_doc;
use crate::{
    enums::ColorPaletteEnum,
    my_assets::MyAssets,
    structs::{
        daily_events::{DailyDiscussionEnum, DailyEvent, DailyEventTypeEnum, DailyEvents},
        general_structs::DailyEventsModalVisible,
        missions::Missions,
        player_stats::PlayerStats,
        trigger_structs::{DailyEventTrigger, MissionReportModalSignButtonTrigger},
    },
    ui::modals::daily_events::spontaneous_application_event_doc::spontaneous_application_event_doc,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
// Show daily events documents on desk
pub fn daily_events_modal(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    // ! WIP - Check that if it works
    query: Query<Entity, With<DailyEventTrigger>>,
    daily_events_modal_visibility: Res<DailyEventsModalVisible>,
    daily_events: Res<DailyEvents>,
    _player_stats: ResMut<PlayerStats>,
    mut _texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Despawn existing modals if visibility is set to false
    if daily_events_modal_visibility.is_changed() && !daily_events_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let daily_events_len = daily_events.0.len();

    // Spawn the mission report modal if visibility is true and there are mission reports
    if daily_events_modal_visibility.is_changed()
        && daily_events_modal_visibility.0
        && daily_events_len > 0
    {
        // let container_image: Handle<Image> =
        //     asset_server.load("images/rooms/barrack/inventory_container.png");

        let last_daily_event: &DailyEvent = match daily_events.get_last_daily_event() {
            Some(report) => report,
            None => return,
        };

        info!("Last daily event: {:?}", last_daily_event);

        match &last_daily_event.daily_event_type {
            DailyEventTypeEnum::Discussion(_) => {
                discussion_event_doc(&mut commands, &my_assets, &last_daily_event);
            }
            DailyEventTypeEnum::SpontaneousApplication(spontaneous_application) => {
                spontaneous_application_event_doc(
                    &mut commands,
                    &my_assets,
                    spontaneous_application.get_spontaneous_application(),
                );
            }
        }
    }
}
