use super::equipments::{Armor, ItemEnum, Scroll, Weapon};
use crate::enums::{ClassEnum, RecruitStateEnum};
use bevy::prelude::{Component, Resource};
use uuid::Uuid;

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct RecruitStats {
    pub class: ClassEnum,
    pub endurance: u16,
    pub experience: u32,
    pub id: Uuid,
    pub image_atlas_index: u16,
    pub intelligence: u16,
    pub level: u8,
    pub max_experience: u32,
    pub name: String,
    pub recruit_inventory: RecruitInventory,
    pub state: RecruitStateEnum,
    pub strength: u16,
}

#[derive(Default, Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedRecruitForEquipment(pub Option<RecruitStats>);

impl SelectedRecruitForEquipment {
    pub fn get_inventory(&self) -> RecruitInventory {
        if let Some(recruit) = &self.0 {
            return recruit.recruit_inventory.clone();
        }

        return RecruitInventory::generate_empty_inventory();
    }

    pub fn get_id(&self) -> Option<Uuid> {
        if let Some(recruit) = &self.0 {
            return Some(recruit.id);
        }

        return None;
    }

    pub fn get_selected_recruit_for_equipment(&self) -> Option<RecruitStats> {
        if let Some(recruit) = &self.0 {
            return Some(recruit.clone());
        }

        return None;
    }
}

#[derive(Default, Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedRecruitForMission(pub Option<RecruitStats>);

impl SelectedRecruitForMission {
    // pub fn get_inventory(&self) -> RecruitInventory {
    //     if let Some(recruit) = &self.0 {
    //         return recruit.recruit_inventory.clone();
    //     }

    //     RecruitInventory::generate_empty_inventory()
    // }

    // pub fn get_id(&self) -> Option<Uuid> {
    //     if let Some(recruit) = &self.0 {
    //         return Some(recruit.id);
    //     }

    //     None
    // }

    // pub fn equip_weapon(&mut self, weapon: Weapon) {
    //     if let Some(recruit) = &mut self.0 {
    //         recruit.recruit_inventory.weapon = Some(weapon);
    //     }
    // }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct RecruitInventory {
    pub armor: Option<Armor>,
    pub weapon: Option<Weapon>,
    pub scrolls: Vec<Scroll>,
}

impl RecruitInventory {
    pub fn generate_empty_inventory() -> Self {
        return Self {
            armor: None,
            weapon: None,
            scrolls: vec![],
        };
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
        self.strength *= 2;
        self.endurance *= 2;
        self.intelligence *= 2;
    }

    pub fn equip_item(&mut self, item: &ItemEnum) {
        match item {
            ItemEnum::Weapon(weapon) => {
                self.recruit_inventory.weapon = Some(weapon.clone());
            }
            ItemEnum::Armor(armor) => {
                self.recruit_inventory.armor = Some(armor.clone());
            }
            ItemEnum::Scroll(scroll, _) => {
                self.recruit_inventory.scrolls.push(scroll.clone());
            }
        }
    }

    pub fn get_additional_strength_from_items(&self) -> u32 {
        let mut additional_strength = 0;

        if let Some(weapon) = &self.recruit_inventory.weapon {
            if let Some(strength) = weapon.strength {
                additional_strength += strength;
            }
        }

        if let Some(armor) = &self.recruit_inventory.armor {
            if let Some(strength) = armor.strength {
                additional_strength += strength;
            }
        }

        for scroll in &self.recruit_inventory.scrolls {
            if let Some(strength) = scroll.strength {
                additional_strength += strength;
            }
        }

        return additional_strength;
    }

    pub fn get_additional_endurance_from_items(&self) -> u32 {
        let mut additional_endurance = 0;

        if let Some(weapon) = &self.recruit_inventory.weapon {
            if let Some(endurance) = weapon.endurance {
                additional_endurance += endurance;
            }
        }

        if let Some(armor) = &self.recruit_inventory.armor {
            if let Some(endurance) = armor.endurance {
                additional_endurance += endurance;
            }
        }

        for scroll in &self.recruit_inventory.scrolls {
            if let Some(endurance) = scroll.endurance {
                additional_endurance += endurance;
            }
        }

        return additional_endurance;
    }

    pub fn get_additional_intelligence_from_items(&self) -> u32 {
        let mut additional_intelligence = 0;

        if let Some(weapon) = &self.recruit_inventory.weapon {
            if let Some(intelligence) = weapon.intelligence {
                additional_intelligence += intelligence;
            }
        }

        if let Some(armor) = &self.recruit_inventory.armor {
            if let Some(intelligence) = armor.intelligence {
                additional_intelligence += intelligence;
            }
        }

        for scroll in &self.recruit_inventory.scrolls {
            if let Some(intelligence) = scroll.intelligence {
                additional_intelligence += intelligence;
            }
        }

        return additional_intelligence;
    }

    pub fn get_total_merged_stats(&self) -> u32 {
        return self.strength as u32
            + self.get_additional_strength_from_items()
            + self.endurance as u32
            + self.get_additional_endurance_from_items()
            + self.intelligence as u32
            + self.get_additional_intelligence_from_items();
    }

    //     pub fn get_total_strength(&self) -> u32 {
    //         return self.strength as u32 + self.get_additional_strength_from_items();
    //     }

    //     pub fn get_total_endurance(&self) -> u32 {
    //         return self.endurance as u32 + self.get_additional_endurance_from_items();
    //     }

    //     pub fn get_total_intelligence(&self) -> u32 {
    //         return self.intelligence as u32 + self.get_additional_intelligence_from_items();
    //     }
}
