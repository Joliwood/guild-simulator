use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    my_assets::MyAssets,
    structs::{
        daily_events::{DailyDiscussionEnum, DailyEvents},
        missions::{MissionReports, Missions},
        player_stats::PlayerStats,
        trigger_structs::{NotificationToastTrigger, SleepButtonTrigger},
    },
    ui::interface::notifications::spawn_or_update_notification::spawn_or_update_notification,
    utils::finish_mission,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn sleep_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &SleepButtonTrigger, &mut BorderColor),
        Changed<Interaction>,
    >,
    mut player_stats: ResMut<PlayerStats>,
    my_assets: Res<MyAssets>,
    mut missions: ResMut<Missions>,
    mut mission_reports: ResMut<MissionReports>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, With<NotificationToastTrigger>>,
    mut windows: Query<&mut Window>,
    mut daily_events: ResMut<DailyEvents>,
) {
    let mut window = windows.single_mut();

    for (interaction, _button, mut border_color) in interaction_query.iter_mut() {
        if !mission_reports.0.is_empty() {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                // Increment the day in player_stats
                player_stats.day += 1;
                play_sound(&my_assets, &mut commands, SoundEnum::KeysRemovedFromDoor);
                play_sound(&my_assets, &mut commands, SoundEnum::CockrelMorning);

                // We iterate on every missions to decrement the days left for every mission that days_left.is_some()
                let mission_ids: Vec<_> = missions
                    .0
                    .iter()
                    .filter(|mission| mission.days_left.is_some())
                    .map(|mission| mission.id)
                    .collect();
                for mission_id in mission_ids {
                    missions.decrement_days_left_by_mission_id(mission_id);
                    let is_mission_over = missions.is_mission_over(mission_id);
                    if is_mission_over {
                        let percent_of_victory =
                            missions.get_percent_of_victory_by_mission_id(mission_id);

                        if percent_of_victory.is_none() {
                            continue;
                        }

                        // WIP - Useless i think, check this
                        let recruit_id = missions.get_recruit_send_id_by_mission_id(mission_id);
                        if recruit_id.is_none() {
                            continue;
                        }

                        finish_mission(
                            &mut player_stats,
                            mission_id,
                            &mut missions,
                            percent_of_victory.unwrap() as f32,
                            &mut mission_reports,
                        );

                        for entity in query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }

                        spawn_or_update_notification(
                            &mut commands,
                            &my_assets,
                            &mut texture_atlas_layouts,
                            &mut mission_reports,
                        );
                    }
                }

                let new_daily_events = daily_events.get_random_number_of_daily_events(8);
                daily_events.0 = new_daily_events;
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Default;
                border_color.0 = Color::NONE;
            }
        }
    }
}
