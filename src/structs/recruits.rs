use super::equipments::{Armor, BonusEnum, ItemEnum, Scroll, Weapon};
use crate::enums::{ClassEnum, RecruitStateEnum};
use bevy::prelude::{Component, Resource};
use uuid::Uuid;

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct RecruitStats {
    pub class: ClassEnum,
    pub experience: u32,
    pub id: Uuid,
    pub image_atlas_index: u16,
    pub level: u8,
    pub max_experience: u32,
    pub name: String,
    pub recruit_inventory: RecruitInventory,
    pub state: RecruitStateEnum,
    pub power: u32,
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

    pub fn get_power_multiplicator_from_optimized_class(&self, class: &ClassEnum) -> f32 {
        let mut multiplicator = 1.0;

        if let Some(weapon) = &self.weapon {
            if weapon.optimized_for.0.contains(class) {
                multiplicator += weapon.optimized_for.1 as f32 / 100.;
            }
        }

        if let Some(armor) = &self.armor {
            if armor.optimized_for.0.contains(class) {
                multiplicator += armor.optimized_for.1 as f32 / 100.;
            }
        }

        for scroll in &self.scrolls {
            if scroll.optimized_for.0.contains(class) {
                multiplicator += scroll.optimized_for.1 as f32 / 100.;
            }
        }

        return multiplicator;
    }

    pub fn get_gold_multiplicator_from_scroll_bonus(&self) -> f32 {
        let mut multiplicator = 1.0;

        for scroll in &self.scrolls {
            for bonus in &scroll.bonus {
                if let BonusEnum::Gold(value) = bonus {
                    multiplicator += *value as f32 / 100.;
                }
            }
        }

        return multiplicator;
    }

    #[allow(dead_code)]
    // WIP - Not tested yet
    pub fn get_experience_multiplicator_from_scroll_bonus(&self) -> f32 {
        let mut multiplicator = 1.0;

        for scroll in &self.scrolls {
            for bonus in &scroll.bonus {
                if let BonusEnum::Experience(value) = bonus {
                    multiplicator += *value as f32 / 100.;
                }
            }
        }

        return multiplicator;
    }

    pub fn get_second_loot_chance_to_additionate_from_scroll_bonus(&self) -> u8 {
        let mut percent_chance_to_add: u8 = 0;

        for scroll in &self.scrolls {
            for bonus in &scroll.bonus {
                if let BonusEnum::LuckyLoot(value) = bonus {
                    percent_chance_to_add += *value;
                }
            }
        }

        return percent_chance_to_add;
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
        self.power *= 2;
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

    pub fn get_additional_power_from_items(&self) -> u32 {
        let mut additional_raw_power = 0;

        if let Some(weapon) = &self.recruit_inventory.weapon {
            additional_raw_power += weapon.power;
        }

        if let Some(armor) = &self.recruit_inventory.armor {
            additional_raw_power += armor.power;
        }

        if let Some(additionl_power_from_scroll) =
            get_total_power_with_additional_power(&self.recruit_inventory.scrolls)
        {
            additional_raw_power += additionl_power_from_scroll;
        }

        let total_recruit_raw_power = self.power + additional_raw_power;

        let multiplicator = self
            .recruit_inventory
            .get_power_multiplicator_from_optimized_class(&self.class);

        let total_recruit_power = (total_recruit_raw_power as f32 * multiplicator) as u32;

        return total_recruit_power - self.power;
    }

    pub fn get_total_power(&self) -> u32 {
        return self.get_additional_power_from_items() + self.power;
    }
}

pub fn get_total_power_with_additional_power(scrolls: &[Scroll]) -> Option<u32> {
    let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

    // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
    let total_raw_power: u32 = bonuses
        .flat_map(|bonus| {
            bonus.iter().map(|b| match b {
                BonusEnum::RawPower(value) => *value,
                _ => 0,
            })
        })
        .sum();

    if total_raw_power > 0 {
        Some(total_raw_power)
    } else {
        None
    }
}
