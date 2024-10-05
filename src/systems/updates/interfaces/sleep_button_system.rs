use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        missions::{MissionReports, Missions},
        player_stats::PlayerStats,
        trigger_structs::SleepButtonTrigger,
    },
    ui::interface::gold_counter::MyAssets,
    utils::finish_mission,
};
use bevy::prelude::*;

pub fn sleep_button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &SleepButtonTrigger), Changed<Interaction>>,
    mut player_stats: ResMut<PlayerStats>,
    my_assets: Res<MyAssets>,
    mut missions: ResMut<Missions>,
    mut mission_reports: ResMut<MissionReports>,
) {
    for (interaction, _button) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            // Increment the day in player_stats
            player_stats.day += 1;
            play_sound(&my_assets, &mut commands, SoundEnum::KeysRemovedFromDoor);
            play_sound(&my_assets, &mut commands, SoundEnum::CockrelMorning);

            // We iterate on every missions to decrement the days left for every mission that days_left.is_some()
            let mission_ids: Vec<_> = missions
                .0
                .iter()
                .filter(|mission| mission.days_left.is_some()) // Only keep missions where recruit_send is Some
                .map(|mission| mission.id)
                .collect();
            for mission_id in mission_ids {
                // info!("WE ARE BEFORE");
                missions.decrement_days_left_by_mission_id(mission_id);
                let is_mission_over = missions.is_mission_over(mission_id);
                // info!("WE ARE AFTER : {}", is_mission_over);
                if is_mission_over {
                    let percent_of_victory =
                        missions.get_percent_of_victory_by_mission_id(mission_id);

                    // let percent_of_victory = selected_mission.percent_of_victory;
                    if percent_of_victory.is_none() {
                        continue;
                    }

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
                }
            }
        }
    }
}
