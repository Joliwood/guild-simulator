use crate::{
    enums::RecruitEnum,
    structs::general_structs::{load_weapon_by_id, PlayerStats, RecruitInventory, RecruitStats},
    systems::recruits::hire_new_recruits::hire_new_recruits,
};
use bevy::prelude::*;
use uuid::Uuid;

/// Initialization of the player's recruits
///
/// Only with first recruits, the stats will be fixed
/// The others recruted will have random stats
pub fn hiring_setup(mut player_stats: ResMut<PlayerStats>) {
    let first_weapon = load_weapon_by_id(1);
    let new_recruits = vec![
        RecruitStats {
            recruit_inventory: RecruitInventory {
                armor: None,
                weapon: first_weapon,
                scrolls: vec![],
            },
            class: RecruitEnum::Warrior,
            endurance: 10,
            experience: 0,
            id: Uuid::new_v4(),
            image_atlas_index: 0,
            intelligence: 5,
            level: 1,
            max_experience: 100,
            name: "Warzazat".to_string(),
            strength: 10,
        },
        RecruitStats {
            recruit_inventory: RecruitInventory::generate_empty_inventory(),
            class: RecruitEnum::Mage,
            endurance: 5,
            experience: 0,
            id: Uuid::new_v4(),
            image_atlas_index: 1,
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
