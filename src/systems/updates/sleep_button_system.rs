use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    structs::{
        general_structs::{Missions, PlayerStats},
        trigger_structs::SleepButtonTrigger,
    },
};
use bevy::prelude::*;

pub fn sleep_button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &SleepButtonTrigger), Changed<Interaction>>,
    mut player_stats: ResMut<PlayerStats>,
    asset_server: Res<AssetServer>,
    mut missions: ResMut<Missions>,
) {
    for (interaction, _button) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            // Increment the day in player_stats
            player_stats.day += 1;
            play_sound(&asset_server, &mut commands, SoundEnum::KeysRemovedFromDoor);
            play_sound(&asset_server, &mut commands, SoundEnum::CockrelMorning);

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

            // We iterate on every missions to decrement the days left for every mission that day_left.is_some()
            info!("Mission AREEEEE {:?}", missions);
            let mission_ids: Vec<_> = missions.0.iter().map(|mission| mission.id).collect();
            for mission_id in mission_ids {
                missions.decrement_days_left_by_mission_id(mission_id);
            }
        }
    }
}
