use super::equipments::{Armor, Item, Scroll, Weapon};
use crate::enums::{RecruitEnum, RecruitStateEnum};
use bevy::prelude::{Component, Resource};
use uuid::Uuid;

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct RecruitStats {
    pub class: RecruitEnum,
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
pub struct SelectedRecruit(pub Option<RecruitStats>);

impl SelectedRecruit {
    pub fn get_inventory(&self) -> RecruitInventory {
        if let Some(recruit) = &self.0 {
            return recruit.recruit_inventory.clone();
        }

        RecruitInventory::generate_empty_inventory()
    }

    pub fn get_id(&self) -> Option<Uuid> {
        if let Some(recruit) = &self.0 {
            return Some(recruit.id);
        }

        None
    }

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        if let Some(recruit) = &mut self.0 {
            recruit.recruit_inventory.weapon = Some(weapon);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RecruitInventory {
    pub armor: Option<Armor>,
    pub weapon: Option<Weapon>,
    pub scrolls: Vec<Scroll>,
}

impl RecruitInventory {
    pub fn generate_empty_inventory() -> Self {
        Self {
            armor: None,
            weapon: None,
            scrolls: vec![],
        }
    }

    pub fn get_weapon(&self) -> Option<Weapon> {
        if let Some(weapon) = &self.weapon {
            return Some(weapon.clone());
        }

        None
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

    pub fn get_item(&self, item: Item) -> Option<Item> {
        match item {
            Item::Weapon(_weapon) => {
                if let Some(weapon) = &self.recruit_inventory.weapon {
                    return Some(Item::Weapon(weapon.clone()));
                }
            }
            Item::Armor(_armor) => {
                if let Some(armor) = &self.recruit_inventory.armor {
                    return Some(Item::Armor(armor.clone()));
                }
            }
            Item::Scroll(_scroll, _) => {
                if let Some(scroll) = self.recruit_inventory.scrolls.first() {
                    return Some(Item::Scroll(scroll.clone(), 1));
                }
            }
        }

        None
    }

    pub fn equip_item(&mut self, item: &Item) {
        match item {
            Item::Weapon(weapon) => {
                self.recruit_inventory.weapon = Some(weapon.clone());
            }
            Item::Armor(armor) => {
                self.recruit_inventory.armor = Some(armor.clone());
            }
            Item::Scroll(scroll, _) => {
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

        additional_strength
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

        additional_endurance
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

        additional_intelligence
    }

    pub fn get_total_merged_stats(&self) -> u32 {
        return self.strength as u32
            + self.get_additional_strength_from_items()
            + self.endurance as u32
            + self.get_additional_endurance_from_items()
            + self.intelligence as u32
            + self.get_additional_intelligence_from_items();
    }

    pub fn get_total_strength(&self) -> u32 {
        return self.strength as u32 + self.get_additional_strength_from_items();
    }

    pub fn get_total_endurance(&self) -> u32 {
        return self.endurance as u32 + self.get_additional_endurance_from_items();
    }

    pub fn get_total_intelligence(&self) -> u32 {
        return self.intelligence as u32 + self.get_additional_intelligence_from_items();
    }
}
