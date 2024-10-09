use crate::{
    // audio::play_sound::play_sound,
    // enums::SoundEnum,
    structs::{
        general_structs::{Missions, PlayerStats, SelectedMission},
        trigger_structs::SleepButtonTrigger,
    },
    utils::finish_mission,
};
use bevy::prelude::*;

pub fn sleep_button_system(
    mut _commands: Commands,
    mut interaction_query: Query<(&Interaction, &SleepButtonTrigger), Changed<Interaction>>,
    mut player_stats: ResMut<PlayerStats>,
    _asset_server: Res<AssetServer>,
    mut missions: ResMut<Missions>,
    selected_mission: Res<SelectedMission>,
) {
    for (interaction, _button) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            // Increment the day in player_stats
            player_stats.day += 1;
            // play_sound(&asset_server, &mut commands, SoundEnum::KeysRemovedFromDoor);
            // play_sound(&asset_server, &mut commands, SoundEnum::CockrelMorning);

            // ! --- OLD CODE --- //
            // ! --- Keep for next feature mission V2 --- //
            // let percent_of_victory =
            //     selected_mission.percent_of_victory.unwrap() as f32;
            // let is_mission_sucess = is_mission_success(percent_of_victory);
            // if is_mission_sucess {
            //     info!("The mission is a success !",);
            //     let mission_ennemy_level =
            //         selected_mission.mission.as_ref().unwrap().level;
            //     let xp_earned = get_xp_earned(mission_ennemy_level);
            //     let gold_earned = (mission_ennemy_level * 10) as i32;
            //     let recruit_id = selected_mission.recruit_id.unwrap();

            //     player_stats.gain_xp_to_recruit(recruit_id, xp_earned);
            //     player_stats.increment_golds(gold_earned);
            // } else {
            //     info!("The mission is a failure !");
            // }

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
                    let percent_of_victory = selected_mission.percent_of_victory;
                    if percent_of_victory.is_none() {
                        continue;
                    }

                    finish_mission(
                        &mut player_stats,
                        mission_id,
                        &mut missions,
                        percent_of_victory.unwrap() as f32,
                    );
                    // let recruit_id = missions.get_recruit_id_send_by_mission_id(mission_id);
                    // // info!(
                    // //     "The mission {} is over, the recruit {:?} is back",
                    // //     mission_id, recruit
                    // // );
                    // if recruit_id.is_none() {
                    //     continue;
                    // }
                    // player_stats
                    //     .update_state_of_recruit(recruit_id.unwrap(), RecruitStateEnum::Available);
                    // missions.desassign_recruit_to_mission(mission_id);
                    // let percent_of_victory = selected_mission.percent_of_victory.unwrap() as f32;
                }
            }
        }
    }
}
