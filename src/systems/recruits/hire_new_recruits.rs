use crate::structs::{PlayerStats, RecruitStats};
use bevy::prelude::ResMut;

pub fn hire_new_recruits(
    mut player_stats: ResMut<PlayerStats>,
    new_recruits: Vec<RecruitStats>,
) -> bool {
    let recruits_number = player_stats.recruits.len();

    for recruit in new_recruits {
        player_stats.recruits.push(recruit);
    }

    println!(
        "Recruits are ready ! :\n {:?} recruits are ready to fight !",
        player_stats.recruits
    );

    if player_stats.recruits.len() == recruits_number {
        return true;
    } else {
        return false;
    }
}
