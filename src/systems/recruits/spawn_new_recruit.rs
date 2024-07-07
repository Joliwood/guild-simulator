use crate::structs::{PlayerStats, RecruitStats};
use bevy::prelude::ResMut;

pub fn spawn_new_recruit(mut player_stats: ResMut<PlayerStats>, new_recruit: RecruitStats) -> bool {
    player_stats.recruits.push(new_recruit);

    println!(
        "Recruits are ready !, {:?} recruits are ready to fight !",
        player_stats.recruits
    );

    return true;
}
