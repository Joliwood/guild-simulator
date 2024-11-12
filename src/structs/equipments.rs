use crate::enums::{ClassEnum, ItemRaretyEnum};
use bevy::prelude::Component;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Component, PartialEq, Eq, Hash)]
pub enum ItemEnum {
    Weapon(Weapon),
    Armor(Armor),
    Scroll(Scroll, u16),
}

impl ItemEnum {
    pub fn get_atlas_index(&self) -> u16 {
        match self {
            ItemEnum::Weapon(weapon) => return weapon.image_atlas_index,
            ItemEnum::Armor(armor) => return armor.image_atlas_index,
            ItemEnum::Scroll(scroll, _) => return scroll.image_atlas_index,
        }
    }

    #[allow(dead_code)]
    pub fn get_item_loot_tooltip_description(&self) -> String {
        match self {
            ItemEnum::Weapon(weapon) => format!(
                "{}\nPrice: {}\nPower: {}",
                weapon.name, weapon.price, weapon.power,
            ),
            ItemEnum::Armor(armor) => format!(
                "{}\nPrice: {}\nPower: {}",
                armor.name, armor.price, armor.power
            ),
            ItemEnum::Scroll(scroll, _) => format!(
                "{}\nPrice: {}\nPower: {:?}",
                scroll.name, scroll.price, scroll.bonus,
            ),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Weapon {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub power: u32,
    pub rarety: ItemRaretyEnum,
    pub optimized_for: (Vec<ClassEnum>, u32),
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Armor {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub rarety: ItemRaretyEnum,
    pub power: u32,
    pub optimized_for: (Vec<ClassEnum>, u32),
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Scroll {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub optimized_for: (Vec<ClassEnum>, u32),
    pub bonus: Vec<BonusEnum>,
}

impl Scroll {
    pub fn get_raw_power_from_bonus(&self) -> u32 {
        let mut power = 0;

        for bonus in &self.bonus {
            match bonus {
                BonusEnum::RawPower(value) => power += value,
                _ => {}
            }
        }

        return power;
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum BonusEnum {
    /// Raw power
    RawPower(u32),

    /// Add % golds for each success mission
    Gold(u32),

    /// increase the chance to earn a second loot in % for each success mission
    LuckyLoot(u32),

    /// Add % experience for each success mission
    Experience(u32),

    /// Increate the recruit's equipment stats by %
    Reinforcement(u32),

    /// Increate the native recruit stats by %
    NaturalGrowth(u32),

    /// Amount of golds it can be increased when a buyer is interested
    Collector(u32),
}
