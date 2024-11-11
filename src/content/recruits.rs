#![allow(dead_code)]

use crate::{
    enums::{ClassEnum, RecruitStateEnum},
    structs::recruits::{RecruitInventory, RecruitStats},
};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RecruitEnum {
    JeanLouisDavid,
    Hubert,
    BigNoob,
}

impl RecruitEnum {
    pub fn get_recruit(&self) -> RecruitStats {
        match self {
            RecruitEnum::JeanLouisDavid => RecruitStats {
                class: ClassEnum::Warrior,
                endurance: 4,
                experience: 0,
                id: Uuid::new_v4(),
                image_atlas_index: 0,
                intelligence: 2,
                level: 1,
                max_experience: 100,
                name: "Jean-Louis-David".to_string(),
                recruit_inventory: RecruitInventory::default(),
                state: RecruitStateEnum::Available,
                strength: 5,
            },
            RecruitEnum::Hubert => RecruitStats {
                class: ClassEnum::Mage,
                endurance: 8,
                experience: 0,
                id: Uuid::new_v4(),
                image_atlas_index: 1,
                intelligence: 0,
                level: 1,
                max_experience: 100,
                name: "Hubert".to_string(),
                recruit_inventory: RecruitInventory::default(),
                state: RecruitStateEnum::Available,
                strength: 12,
            },
            RecruitEnum::BigNoob => RecruitStats {
                class: ClassEnum::Rogue,
                endurance: 5,
                experience: 0,
                id: Uuid::new_v4(),
                image_atlas_index: 3,
                intelligence: 1,
                level: 1,
                max_experience: 100,
                name: "Big noob".to_string(),
                recruit_inventory: RecruitInventory::default(),
                state: RecruitStateEnum::Available,
                strength: 5,
            },
        }
    }
}
