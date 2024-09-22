#![allow(dead_code, unused_imports)]
use super::equipments::{Item, Weapon};
use crate::{
    enums::{RecruitEnum, RoomEnum},
    structs::equipments::Weapons,
    utils::format_ron_equipments_for_display,
};
use bevy::{
    log::info,
    prelude::{Component, Resource},
};
use ron::de::from_str;
use serde::Deserialize;
use std::fs;
use uuid::Uuid;

#[derive(Component, Resource)]
pub struct MissionModalVisible(pub bool);

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Component, Resource)]
pub struct PlayerStats {
    pub experience: u32,
    pub golds: i32,
    pub guild_level: i8,
    pub inventory: Vec<Item>,
    pub max_experience: u32,
    pub max_inventory_size: usize,
    pub recruits: Vec<RecruitStats>,
    pub room: RoomEnum,
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct RecruitStats {
    pub class: RecruitEnum,
    pub endurance: u16,
    pub experience: u32,
    pub id: Uuid,
    pub intelligence: u16,
    pub level: u8,
    pub max_experience: u32,
    pub strength: u16,
}

#[derive(Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedRecruit(pub Option<RecruitStats>);

#[derive(Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedMission {
    pub mission: Option<Mission>,
    pub percent_of_victory: Option<u32>,
    pub recruit_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
    pub endurance: u16,
    pub experience: u32,
    pub intelligence: u16,
    pub level: u8,
    pub name: String,
    pub strength: u16,
}

#[derive(Component, Resource)]
pub struct Missions(pub Vec<Mission>);
#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Mission {
    pub ennemy: Ennemy,
    pub id: Uuid,
    pub level: u8,
    pub name: String,
}

// --- Implementations --- //

impl Default for SelectedRecruit {
    fn default() -> Self {
        Self(None)
    }
}

impl Default for SelectedMission {
    fn default() -> Self {
        Self {
            mission: None,
            percent_of_victory: None,
            recruit_id: None,
        }
    }
}

impl PlayerStats {
    pub fn increment_golds(&mut self, amount: i32) {
        self.golds += amount;
    }

    fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        // Reset the experience with left experience after leveling up
        // Then level up
        if self.experience >= self.max_experience {
            self.experience -= self.max_experience;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.guild_level += 1;
        // Set the max experience to the current experience * 2
        self.max_experience *= 2;
    }
}

impl RecruitStats {
    pub fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        // Reset the experience with left experience after leveling up
        // Then level up
        if self.experience >= self.max_experience {
            self.experience -= self.max_experience;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        // Set the max experience to the current experience * 2
        self.max_experience *= 2;
    }
}

fn load_weapon_by_id(id: u16) -> Option<Weapon> {
    let weapons_data = fs::read_to_string("src/data/equipments/weapons.ron")
        .expect("Failed to read the RON file.");

    let weapons: Weapons = from_str(&weapons_data).expect("Failed to deserialize RON data.");

    if let Some(weapon) = weapons.items.iter().find(|weapon| weapon.id == id) {
        info!("Weapon with id = {}: {:?}", id, weapon);
        return Some(weapon.clone());
    } else {
        info!("Weapon with id = {} not found.", id);
        return None;
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        let mut inventory = vec![];
        let first_weapon = load_weapon_by_id(1);
        let second_weapon = load_weapon_by_id(5);

        if let Some(first_weapon) = first_weapon {
            inventory.push(Item::Weapon(first_weapon));
        }

        if let Some(second_weapon) = second_weapon {
            inventory.push(Item::Weapon(second_weapon));
        }

        Self {
            experience: 0,
            golds: 0,
            guild_level: 1,
            inventory,
            max_experience: 100,
            max_inventory_size: 20,
            recruits: vec![],
            room: RoomEnum::Barrack,
        }
    }
}

impl Default for Missions {
    fn default() -> Self {
        Self(vec![
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 1".to_string(),
                level: 1,
                ennemy: Ennemy {
                    endurance: 10,
                    experience: 0,
                    intelligence: 5,
                    level: 1,
                    name: "Ennemy 1".to_string(),
                    strength: 10,
                },
            },
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 2".to_string(),
                level: 2,
                ennemy: Ennemy {
                    endurance: 15,
                    experience: 0,
                    intelligence: 7,
                    level: 2,
                    name: "Ennemy 2".to_string(),
                    strength: 15,
                },
            },
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 3".to_string(),
                level: 3,
                ennemy: Ennemy {
                    endurance: 20,
                    experience: 0,
                    intelligence: 10,
                    level: 3,
                    name: "Ennemy 3".to_string(),
                    strength: 20,
                },
            },
        ])
    }
}
