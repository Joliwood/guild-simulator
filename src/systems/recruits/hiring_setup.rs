use crate::{
    enums::RecruitEnum,
    structs::general_structs::{PlayerStats, RecruitStats},
    systems::recruits::hire_new_recruits::hire_new_recruits,
};
use bevy::prelude::*;
use uuid::Uuid;

/// Initialization of the player's recruits
///
/// Only with first recruits, the stats will be fixed
/// The others recruted will have random stats
pub fn hiring_setup(mut player_stats: ResMut<PlayerStats>) {
    let new_recruits = vec![
        RecruitStats {
            class: RecruitEnum::Warrior,
            endurance: 10,
            experience: 0,
            id: Uuid::new_v4(),
            intelligence: 5,
            level: 1,
            max_experience: 100,
            name: "Warzazat".to_string(),
            strength: 10,
        },
        RecruitStats {
            class: RecruitEnum::Mage,
            endurance: 5,
            experience: 0,
            id: Uuid::new_v4(),
            intelligence: 12,
            level: 1,
            max_experience: 100,
            name: "Wagaly".to_string(),
            strength: 2,
        },
    ];

    hire_new_recruits(&mut player_stats, new_recruits);

    info!("Recruits are ready!");
}
