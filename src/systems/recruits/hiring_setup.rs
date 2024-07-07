use crate::{
    enums::RecruitEnum,
    structs::{PlayerStats, RecruitStats},
    systems::recruits::hire_new_recruits::hire_new_recruits,
};
use bevy::prelude::*;

/// Initialization of the player's recruits
///
/// Only with first recruits, the stats will be fixed
/// The others recruted will have random stats
pub fn hiring_setup(player_stats: ResMut<PlayerStats>) {
    println!(
        "We are ready to hire some recruits ! {}",
        player_stats.recruits.len()
    );

    let new_recruits = vec![
        RecruitStats {
            class: RecruitEnum::Warrior,
            endurance: 10,
            experience: 0,
            intelligence: 5,
            level: 1,
            max_experience: 100,
            strength: 10,
        },
        RecruitStats {
            class: RecruitEnum::Mage,
            endurance: 5,
            experience: 0,
            intelligence: 12,
            level: 1,
            max_experience: 100,
            strength: 2,
        },
    ];

    hire_new_recruits(player_stats, new_recruits);

    println!("Recruits are ready !");
}
