use crate::structs::general_structs::{PlayerStats, RecruitStats};

pub fn hire_new_recruits(player_stats: &mut PlayerStats, new_recruits: Vec<RecruitStats>) -> bool {
    let recruits_number = player_stats.recruits.len();

    for recruit in new_recruits {
        player_stats.recruits.push(recruit);
    }

    return player_stats.recruits.len() == recruits_number;
}
