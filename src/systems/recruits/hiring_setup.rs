use crate::{
    enums::RecruitEnum,
    structs::{PlayerStats, RecruitStats},
    systems::recruits::hire_new_recruits::hire_new_recruits,
};
use bevy::prelude::*;
use uuid::Uuid;

/// Initialization of the player's recruits
///
/// Only with first recruits, the stats will be fixed
/// The others recruted will have random stats
pub fn hiring_setup(mut player_stats: ResMut<PlayerStats>) {
    println!(
        "We are ready to hire some recruits! Currently, we have {} recruits.",
        player_stats.recruits.len()
    );

    let new_recruits = vec![
        RecruitStats {
            id: Uuid::new_v4(),
            class: RecruitEnum::Warrior,
            endurance: 10,
            experience: 0,
            intelligence: 5,
            level: 1,
            max_experience: 100,
            strength: 10,
        },
        RecruitStats {
            id: Uuid::new_v4(),
            class: RecruitEnum::Mage,
            endurance: 5,
            experience: 0,
            intelligence: 12,
            level: 1,
            max_experience: 100,
            strength: 2,
        },
    ];

    hire_new_recruits(&mut player_stats, new_recruits);

    println!("Recruits are ready!");
}
