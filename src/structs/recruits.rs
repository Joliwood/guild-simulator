use super::equipments::{Armor, BonusEnum, ItemEnum, Scroll, Weapon};
use crate::{
    enums::{ClassEnum, RecruitStateEnum},
    utils::merge_multiplicators,
};
use bevy::{
    log::info,
    prelude::{Component, Resource},
};
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

    pub fn get_attack_multiplicator_from_optimized_class(&self, class: &ClassEnum) -> f64 {
        let mut multiplicator = 1.0;

        if let Some(weapon) = &self.weapon {
            if weapon.optimized_for.0.contains(class) {
                multiplicator += weapon.optimized_for.1 as f64 / 100.;
            }
        }

        if let Some(armor) = &self.armor {
            if armor.optimized_for.0.contains(class) {
                multiplicator += armor.optimized_for.1 as f64 / 100.;
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

    pub fn get_experience_multiplicator_from_scroll_bonus(&self) -> f64 {
        let mut multiplicator = 1.0;

        for scroll in &self.scrolls {
            for bonus in &scroll.bonus {
                if let BonusEnum::Experience(value) = bonus {
                    multiplicator += *value as f64 / 100.;
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

    pub fn get_equipment_stats_multiplicator_from_scroll_bonus(&self) -> f64 {
        let mut multiplicator = 1.0;

        for scroll in &self.scrolls {
            for bonus in &scroll.bonus {
                if let BonusEnum::EnhanceEquipment(value) = bonus {
                    multiplicator += *value as f64 / 100.;
                }
            }
        }

        return multiplicator;
    }

    pub fn get_raw_recruit_stats_from_scroll_bonus(&self) -> (f64, f64) {
        let mut additional_raw_attack = 0;
        let mut additional_raw_defense = 0;

        for scroll in &self.scrolls {
            for bonus in &scroll.bonus {
                if let BonusEnum::NaturalRawAttack(value) = bonus {
                    additional_raw_attack += value;
                }
                if let BonusEnum::NaturalRawDefense(value) = bonus {
                    additional_raw_defense += value;
                }
            }
        }

        return (additional_raw_attack as f64, additional_raw_defense as f64);
    }

    pub fn get_recruit_stats_multiplicator_from_scroll_bonus(&self) -> f64 {
        let mut multiplicator = 1.0;

        for scroll in &self.scrolls {
            for bonus in &scroll.bonus {
                if let BonusEnum::NaturalGrowth(value) = bonus {
                    multiplicator += *value as f64 / 100.;
                }
            }
        }

        return multiplicator;
    }

    fn get_stats_from_weapon(&self) -> (u32, u32) {
        let mut weapon_stats = (0, 0);

        if let Some(weapon) = &self.weapon {
            if let Some(attack) = weapon.attack {
                weapon_stats.0 += attack;
            }
            if let Some(defense) = weapon.defense {
                weapon_stats.1 += defense;
            }
        }

        return weapon_stats;
    }

    fn get_stats_from_armor(&self) -> (u32, u32) {
        let mut armor_stats = (0, 0);

        if let Some(armor) = &self.armor {
            if let Some(attack) = armor.attack {
                armor_stats.0 += attack;
            }
            if let Some(defense) = armor.defense {
                armor_stats.1 += defense;
            }
        }

        return armor_stats;
    }

    fn get_stats_from_scrolls(&self) -> (u32, u32) {
        let mut scrolls_stats = (0, 0);

        for scroll in &self.scrolls {
            if let Some(attack) = scroll.attack {
                scrolls_stats.0 += attack;
            }
            if let Some(defense) = scroll.defense {
                scrolls_stats.1 += defense;
            }
        }

        return scrolls_stats;
    }

    pub fn get_stats_from_items(&self) -> (f64, f64) {
        let weapon_stats = self.get_stats_from_weapon();
        let armor_stats = self.get_stats_from_armor();
        let scrolls_stats = self.get_stats_from_scrolls();

        return (
            (weapon_stats.0 + armor_stats.0 + scrolls_stats.0) as f64,
            (weapon_stats.1 + armor_stats.1 + scrolls_stats.1) as f64,
        );
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
        //     let mut additional_raw_defense = 0;
        //     let mut additional_raw_attack = 0;

        //     if let Some(weapon) = &self.recruit_inventory.weapon {
        //         if let Some(attack) = weapon.attack {
        //             additional_attack += attack;
        //         }

        //         if let Some(defense) = weapon.defense {
        //             additional_raw_defense += defense;
        //         }
        //     }

        //     if let Some(armor) = &self.recruit_inventory.armor {
        //         if let Some(attack) = armor.attack {
        //             additional_attack += attack;
        //         }

        //         if let Some(defense) = armor.defense {
        //             additional_raw_defense += defense;
        //         }
        //     }

        //     for scroll in &self.recruit_inventory.scrolls {
        //         if let Some(attack) = scroll.attack {
        //             additional_attack += attack;
        //         }

        //         if let Some(defense) = scroll.defense {
        //             additional_raw_defense += defense;
        //         }

        //         if let Some(additional_raw_attack_from_scroll) =
        //             get_total_raw_attack_with_additional_attack(&self.recruit_inventory.scrolls)
        //         {
        //             additional_attack += additional_raw_attack_from_scroll;
        //         }

        //         if let Some(additional_raw_defense_from_scroll) =
        //             get_total_raw_defense_from_scrolls(&self.recruit_inventory.scrolls)
        //         {
        //             additional_raw_defense += additional_raw_defense_from_scroll;
        //         }
        //     }

        //     let total_recruit_attack = self.attack + additional_raw_attack;

        //     let attack_multiplicator_from_optimized_class = self
        //         .recruit_inventory
        //         .get_attack_multiplicator_from_optimized_class(&self.class);

        //     let defense_multiplicator_from_optimized_class = self
        //         .recruit_inventory
        //         .get_attack_multiplicator_from_optimized_class(&self.class);

        //     let equipment_stats_multiplicator = self
        //         .recruit_inventory
        //         .get_equipment_stats_multiplicator_from_scroll_bonus();

        //     let attack_multiplicator = merge_multiplicators(vec![
        //         attack_multiplicator_from_optimized_class,
        //         equipment_stats_multiplicator,
        //     ]);

        //     let additional_attack =
        //         ((total_recruit_attack as f64 - self.attack as f64) * attack_multiplicator) as u32;

        //     let additional_defense =
        //         (additional_raw_defense as f64 * equipment_stats_multiplicator) as u32;

        //     return (additional_attack, additional_defense);

        let total_stats = self.get_total_stats();

        return (total_stats.0 - self.attack, total_stats.1 - self.defense);
    }

    pub fn get_total_stats(&self) -> (u32, u32) {
        let mut total_recruit_stats = (self.attack as f64, self.defense as f64);
        let mut total_equipment_stats = self.recruit_inventory.get_stats_from_items();
        let mut total_stats = (0., 0.);
        // let additionnal_stats_from_items = self.get_additional_stats_from_items();
        // return (
        //     additionnal_stats_from_items.0 + self.attack,
        //     additionnal_stats_from_items.1 + self.defense,
        // );

        // Section recrue
        // 1 - AJoute les stats brutes natives à la recrue
        // 2 - Ajoute le bonus natif en % à la recrue

        // Section équipement
        // 3 - Ajoute les bonus en % des items

        // Section globale
        // 4 - On merge les stats recrue et équipement
        // 5 - On ajoute le bonus optimized_for en % sur le total (on favorise l'opti des classes)

        // --- //

        // 1
        let raw_stats_from_recruit_natural_bonus = self
            .recruit_inventory
            .get_raw_recruit_stats_from_scroll_bonus();

        total_recruit_stats.0 += raw_stats_from_recruit_natural_bonus.0;
        total_recruit_stats.1 += raw_stats_from_recruit_natural_bonus.1;

        // 2
        let recruit_stats_multiplicator = self
            .recruit_inventory
            .get_recruit_stats_multiplicator_from_scroll_bonus();

        total_recruit_stats.0 *= recruit_stats_multiplicator;
        total_recruit_stats.1 *= recruit_stats_multiplicator;

        // 3
        let equipment_stats_multiplicator_from_scroll_bonus = self
            .recruit_inventory
            .get_equipment_stats_multiplicator_from_scroll_bonus();

        total_equipment_stats.0 *= equipment_stats_multiplicator_from_scroll_bonus;
        total_equipment_stats.1 *= equipment_stats_multiplicator_from_scroll_bonus;

        info!("total_equipment_stats is : {:?}", total_equipment_stats);

        // 4
        total_stats.0 = total_recruit_stats.0 + total_equipment_stats.0;
        total_stats.1 = total_recruit_stats.1 + total_equipment_stats.1;

        // 5
        let attack_multiplicator_from_optimized_class = self
            .recruit_inventory
            .get_attack_multiplicator_from_optimized_class(&self.class);

        total_stats.0 *= attack_multiplicator_from_optimized_class;

        info!("Total stats for recruit {} : {:?}", self.name, total_stats);

        return (total_stats.0 as u32, total_stats.1 as u32);
    }
}

// pub fn get_total_raw_attack_with_additional_attack(scrolls: &[Scroll]) -> Option<u32> {
//     let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

//     // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
//     let total_raw_attack: u32 = bonuses
//         .flat_map(|bonus| {
//             bonus.iter().map(|b| match b {
//                 BonusEnum::RawAttack(value) => *value,
//                 _ => 0,
//             })
//         })
//         .sum();

//     if total_raw_attack > 0 {
//         Some(total_raw_attack)
//     } else {
//         None
//     }
// }

// pub fn get_total_raw_defense_from_scrolls(scrolls: &[Scroll]) -> Option<u32> {
//     let bonuses = scrolls.iter().map(|scroll| &scroll.bonus);

//     // Iterate over the bonuses and each time we have a RawPower bonus, we add the value to the total power
//     let total_raw_defense: u32 = bonuses
//         .flat_map(|bonus| {
//             bonus.iter().map(|b| match b {
//                 BonusEnum::NaturalRawDefense(value) => *value,
//                 _ => 0,
//             })
//         })
//         .sum();

//     if total_raw_defense > 0 {
//         Some(total_raw_defense)
//     } else {
//         None
//     }
// }
