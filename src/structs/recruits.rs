use super::equipments::{Armor, BonusEnum, ItemEnum, Scroll, Weapon};
use crate::enums::{ClassEnum, RecruitStateEnum};
use bevy::prelude::{Component, Resource};
use uuid::Uuid;

#[derive(Default, Debug, Component, Clone, Eq, PartialEq, Hash)]
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
    pub attack: u32,
    pub defense: u32,
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
                // multiplicator.1 += weapon.optimized_for.1 .1 as f32 / 100.;
            }
        }

        if let Some(armor) = &self.armor {
            if armor.optimized_for.0.contains(class) {
                multiplicator += armor.optimized_for.1 as f32 / 100.;
                // multiplicator.1 += armor.optimized_for.1 .1 as f32 / 100.;
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
        self.attack *= 2;
        self.defense *= 2;
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

    /// return in a tuple :
    /// - additional attack
    /// - additional defense
    pub fn get_additional_stats_from_items(&self) -> (u32, u32) {
        let mut additional_raw_defense = 0;
        let mut additional_attack = 0;

        if let Some(weapon) = &self.recruit_inventory.weapon {
            if let Some(attack) = weapon.attack {
                additional_attack += attack;
            }

            if let Some(defense) = weapon.defense {
                additional_raw_defense += defense;
            }
        }

        if let Some(armor) = &self.recruit_inventory.armor {
            if let Some(attack) = armor.attack {
                additional_attack += attack;
            }

            if let Some(defense) = armor.defense {
                additional_raw_defense += defense;
            }
        }

        for scroll in &self.recruit_inventory.scrolls {
            if let Some(attack) = scroll.attack {
                additional_attack += attack;
            }

            if let Some(defense) = scroll.defense {
                additional_raw_defense += defense;
            }

            if let Some(additional_raw_attack_from_scroll) =
                get_total_raw_attack_with_additional_attack(&self.recruit_inventory.scrolls)
            {
                additional_attack += additional_raw_attack_from_scroll;
            }

            if let Some(additional_raw_defense_from_scroll) =
                get_total_raw_defense_from_scrolls(&self.recruit_inventory.scrolls)
            {
                additional_raw_defense += additional_raw_defense_from_scroll;
            }
        }

        let total_recruit_attack = self.attack + additional_attack;

        let multiplicator = self
            .recruit_inventory
            .get_power_multiplicator_from_optimized_class(&self.class);

        let additional_attack = (total_recruit_attack as f32 * multiplicator) as u32 - self.attack;

        return (additional_attack, additional_raw_defense);
    }

    pub fn get_total_stats(&self) -> (u32, u32) {
        let additionnal_stats_from_items = self.get_additional_stats_from_items();
        return (
            additionnal_stats_from_items.0 + self.attack,
            additionnal_stats_from_items.1 + self.defense,
        );
    }
}

pub fn get_total_raw_attack_with_additional_attack(scrolls: &[Scroll]) -> Option<u32> {
    let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

    // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
    let total_raw_attack: u32 = bonuses
        .flat_map(|bonus| {
            bonus.iter().map(|b| match b {
                BonusEnum::RawAttack(value) => *value,
                _ => 0,
            })
        })
        .sum();

    if total_raw_attack > 0 {
        Some(total_raw_attack)
    } else {
        None
    }
}

pub fn get_total_raw_defense_from_scrolls(scrolls: &[Scroll]) -> Option<u32> {
    let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

    // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
    let total_raw_defense: u32 = bonuses
        .flat_map(|bonus| {
            bonus.iter().map(|b| match b {
                BonusEnum::NaturalRawDefense(value) => *value,
                _ => 0,
            })
        })
        .sum();

    if total_raw_defense > 0 {
        Some(total_raw_defense)
    } else {
        None
    }
}
