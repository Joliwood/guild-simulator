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
                "{}\nPrice: {}\nAttack: {}\n Defense: {}",
                weapon.name,
                weapon.price,
                weapon.attack.unwrap_or(0),
                weapon.defense.unwrap_or(0)
            ),
            ItemEnum::Armor(armor) => format!(
                "{}\nPrice: {}\nAttack: {}\n Defense: {}",
                armor.name,
                armor.price,
                armor.attack.unwrap_or(0),
                armor.defense.unwrap_or(0)
            ),
            ItemEnum::Scroll(scroll, _) => format!(
                "{}\nPrice: {}\nAttack: {}\n Defense: {}",
                scroll.name,
                scroll.price,
                scroll.attack.unwrap_or(0),
                scroll.defense.unwrap_or(0)
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
    pub attack: Option<u32>,
    pub defense: Option<u32>,
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
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    pub optimized_for: (Vec<ClassEnum>, u32),
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Scroll {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub attack: Option<u32>,
    pub defense: Option<u32>,
    pub bonus: Vec<BonusEnum>,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum BonusEnum {
    /// Raw attack
    RawAttack(u32),

    /// Add % golds for each success mission
    Gold(u32),

    /// Increase the chance to earn a second loot in % for each success mission
    LuckyLoot(u8),

    /// Add % experience for each success mission
    Experience(u32),

    /// Increate the recruit's equipment (all of them) stats by %
    Reinforcement(u32),

    /// Increate the native recruit stats (all of them) by %
    NaturalGrowth(u32),

    /// Amount of golds it can be increased when a buyer is interested
    Collector(u32),

    /// Increase the native recruit defense
    NaturalRawDefense(u32),
}
