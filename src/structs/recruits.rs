use super::equipments::{Armor, BonusEnum, ItemEnum, Scroll, Weapon};
use crate::enums::{ClassEnum, RecruitStateEnum};
use bevy::{
    log::info,
    prelude::{Component, Resource},
};
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
    pub physical_power: u32,
    pub magical_power: u32,
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

    pub fn get_power_multiplicator_from_optimized_class(&self, class: &ClassEnum) -> (f32, f32) {
        let mut multiplicator = (1.0, 1.0);

        if let Some(weapon) = &self.weapon {
            if weapon.optimized_for.0.contains(class) {
                multiplicator.0 += weapon.optimized_for.1 .0 as f32 / 100.;
                multiplicator.1 += weapon.optimized_for.1 .1 as f32 / 100.;
            }
        }

        if let Some(armor) = &self.armor {
            if armor.optimized_for.0.contains(class) {
                multiplicator.0 += armor.optimized_for.1 .0 as f32 / 100.;
                multiplicator.1 += armor.optimized_for.1 .1 as f32 / 100.;
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
        self.physical_power *= 2;
        self.magical_power *= 2;
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
    /// - additional physical power
    /// - additional magical power
    /// - additional defense
    pub fn get_additional_stats_from_items(&self) -> (u32, u32, u32) {
        let mut additional_physical_raw_power = 0;
        let mut additional_magical_raw_power = 0;
        let mut additional_raw_defense = 0;

        if let Some(weapon) = &self.recruit_inventory.weapon {
            if let Some(physical_power) = weapon.physical_power {
                additional_physical_raw_power += physical_power;
            }

            if let Some(magical_power) = weapon.magical_power {
                additional_magical_raw_power += magical_power;
            }

            if let Some(defense) = weapon.defense {
                additional_raw_defense += defense;
            }
        }

        if let Some(armor) = &self.recruit_inventory.armor {
            if let Some(physical_power) = armor.physical_power {
                additional_physical_raw_power += physical_power;
            }

            if let Some(magical_power) = armor.magical_power {
                additional_magical_raw_power += magical_power;
            }

            if let Some(defense) = armor.defense {
                additional_raw_defense += defense;
            }
        }

        for scroll in &self.recruit_inventory.scrolls {
            if let Some(physical_power) = scroll.physical_power {
                additional_physical_raw_power += physical_power;
            }

            if let Some(magical_power) = scroll.magical_power {
                additional_magical_raw_power += magical_power;
            }

            if let Some(defense) = scroll.defense {
                additional_raw_defense += defense;
            }

            if let Some(additional_physical_raw_power_from_scroll) =
                get_total_physical_raw_power_with_additional_power(&self.recruit_inventory.scrolls)
            {
                additional_physical_raw_power += additional_physical_raw_power_from_scroll;
            }

            if let Some(additional_magical_raw_power_from_scroll) =
                get_total_magical_raw_power_with_additional_power(&self.recruit_inventory.scrolls)
            {
                additional_magical_raw_power += additional_magical_raw_power_from_scroll;
            }

            if let Some(additional_raw_defense_from_scroll) =
                get_total_raw_defense_from_scrolls(&self.recruit_inventory.scrolls)
            {
                additional_raw_defense += additional_raw_defense_from_scroll;
            }
        }

        let total_recruit_physical_raw_power = self.physical_power + additional_physical_raw_power;
        let total_recruit_magical_raw_power = self.magical_power + additional_magical_raw_power;

        let multiplicator = self
            .recruit_inventory
            .get_power_multiplicator_from_optimized_class(&self.class);

        let additional_physical_power = (total_recruit_physical_raw_power as f32 * multiplicator.0)
            as u32
            - self.physical_power;

        let additional_magical_power =
            (total_recruit_magical_raw_power as f32 * multiplicator.1) as u32 - self.magical_power;

        return (
            additional_physical_power,
            additional_magical_power,
            additional_raw_defense,
        );
    }

    pub fn get_total_power(&self) -> u32 {
        // return self.get_additional_power_from_items() + self.power;
        let additionnal_stats_from_items = self.get_additional_stats_from_items();
        return additionnal_stats_from_items.0
            + self.physical_power
            + additionnal_stats_from_items.1
            + self.magical_power
            + additionnal_stats_from_items.2
            + self.defense;
    }
}

// #[deprecated]
// pub fn get_total_power_with_additional_power(scrolls: &[Scroll]) -> Option<u32> {
//     let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

//     // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
//     let total_raw_power: u32 = bonuses
//         .flat_map(|bonus| {
//             bonus.iter().map(|b| match b {
//                 BonusEnum::RawPower(value) => *value,
//                 _ => 0,
//             })
//         })
//         .sum();

//     if total_raw_power > 0 {
//         Some(total_raw_power)
//     } else {
//         None
//     }
// }

pub fn get_total_physical_raw_power_with_additional_power(scrolls: &[Scroll]) -> Option<u32> {
    let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

    // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
    let total_physical_raw_power: u32 = bonuses
        .flat_map(|bonus| {
            bonus.iter().map(|b| match b {
                BonusEnum::PhysicalRawPower(value) => *value,
                _ => 0,
            })
        })
        .sum();

    info!("total_physical_raw_power: {}", total_physical_raw_power);

    if total_physical_raw_power > 0 {
        Some(total_physical_raw_power)
    } else {
        None
    }
}

pub fn get_total_magical_raw_power_with_additional_power(scrolls: &[Scroll]) -> Option<u32> {
    let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

    // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
    let total_magical_raw_power: u32 = bonuses
        .flat_map(|bonus| {
            bonus.iter().map(|b| match b {
                BonusEnum::MagicalRawPower(value) => *value,
                _ => 0,
            })
        })
        .sum();

    if total_magical_raw_power > 0 {
        Some(total_magical_raw_power)
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
