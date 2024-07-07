use crate::{
    enums::RecruitEnum,
    structs::{PlayerStats, RecruitStats},
    systems::recruits::spawn_new_recruit::spawn_new_recruit,
};
use bevy::prelude::*;

/// Initialization of the player's recruits
///
/// Only with first recruits, the stats will be fixed
/// The others recruted will have random stats
pub fn recruit_setup(player_stats: ResMut<PlayerStats>) {
    println!(
        "We are ready to recrut some troops ! {}",
        player_stats.recruits.len()
    );

    spawn_new_recruit(
        player_stats,
        RecruitStats {
            class: RecruitEnum::Warrior,
            endurance: 10,
            experience: 0,
            intelligence: 5,
            level: 1,
            max_experience: 100,
            strength: 10,
        },
    );

    // TODO - Fix the fact we cannot reuse player_stats to create a second recruit

    // spawn_new_recruit(
    //     player_stats,
    //     RecruitStats {
    //         class: RecruitEnum::Mage,
    //         endurance: 5,
    //         experience: 0,
    //         intelligence: 12,
    //         level: 1,
    //         max_experience: 100,
    //         strength: 2,
    //     },
    // );

    println!("Recruits are ready !");
}
