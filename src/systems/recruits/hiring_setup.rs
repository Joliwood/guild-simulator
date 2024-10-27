use crate::{
    content::equipments::{armors::ArmorsEnum, weapons::WeaponsEnum},
    enums::{RecruitEnum, RecruitStateEnum},
    structs::{
        general_structs::{load_armor, load_weapon},
        player_stats::PlayerStats,
        recruits::{RecruitInventory, RecruitStats},
    },
    systems::recruits::hire_new_recruits::hire_new_recruits,
};
use bevy::prelude::*;
use uuid::Uuid;

/// Initialization of the player's recruits
///
/// Only with first recruits, the stats will be fixed
/// The others recruted will have random stats
pub fn hiring_setup(mut player_stats: ResMut<PlayerStats>) {
    let first_weapon = load_weapon(WeaponsEnum::SpearOfDestiny);
    let first_armor = load_armor(ArmorsEnum::GauntletsOfPower);
    let new_recruits = vec![
        RecruitStats {
            class: RecruitEnum::Warrior,
            endurance: 10,
            experience: 0,
            id: Uuid::new_v4(),
            image_atlas_index: 0,
            intelligence: 5,
            level: 1,
            max_experience: 100,
            name: "Warzazat".to_string(),
            recruit_inventory: RecruitInventory {
                armor: Some(first_armor),
                weapon: Some(first_weapon),
                scrolls: vec![],
            },
            state: RecruitStateEnum::Available,
            strength: 25,
        },
        RecruitStats {
            class: RecruitEnum::Mage,
            endurance: 5,
            experience: 0,
            id: Uuid::new_v4(),
            image_atlas_index: 1,
            intelligence: 12,
            level: 1,
            max_experience: 100,
            name: "Wagaly".to_string(),
            recruit_inventory: RecruitInventory::generate_empty_inventory(),
            state: RecruitStateEnum::Available,
            strength: 2,
        },
    ];

    hire_new_recruits(&mut player_stats, new_recruits);

    info!("Recruits are ready!");
}
