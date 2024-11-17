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
                "{}\nPrice: {}\nPhysical power: {}\n Magical power: {}\n Defense: {}",
                weapon.name,
                weapon.price,
                weapon.physical_power.unwrap_or(0),
                weapon.magical_power.unwrap_or(0),
                weapon.defense.unwrap_or(0)
            ),
            ItemEnum::Armor(armor) => format!(
                "{}\nPrice: {}\nPhysical power: {}\n Magical power: {}\n Defense: {}",
                armor.name,
                armor.price,
                armor.physical_power.unwrap_or(0),
                armor.magical_power.unwrap_or(0),
                armor.defense.unwrap_or(0)
            ),
            ItemEnum::Scroll(scroll, _) => format!(
                "{}\nPrice: {}\nPhysical power: {}\n Magical power: {}\n Defense: {}",
                scroll.name,
                scroll.price,
                scroll.physical_power.unwrap_or(0),
                scroll.magical_power.unwrap_or(0),
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
    pub physical_power: Option<u32>,
    pub magical_power: Option<u32>,
    pub defense: Option<u32>,
    // pub power: u32,
    pub rarety: ItemRaretyEnum,
    /// The first tuple is physical, the second is magic
    pub optimized_for: (Vec<ClassEnum>, (u32, u32)),
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Armor {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub rarety: ItemRaretyEnum,
    pub physical_power: Option<u32>,
    pub magical_power: Option<u32>,
    pub defense: Option<u32>,
    // pub power: u32,
    /// The first tuple is physical, the second is magic
    pub optimized_for: (Vec<ClassEnum>, (u32, u32)),
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Scroll {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub physical_power: Option<u32>,
    pub magical_power: Option<u32>,
    pub defense: Option<u32>,
    pub bonus: Vec<BonusEnum>,
}

// impl Scroll {
//     #[allow(dead_code)]
//     pub fn get_raw_power_from_bonus(&self) -> u32 {
//         let mut power = 0;

//         for bonus in &self.bonus {
//             if let BonusEnum::RawPower(value) = bonus {
//                 power += value;
//             }
//         }

//         return power;
//     }
// }

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum BonusEnum {
    // TODO - To check the logic
    /// Physical raw power
    PhysicalRawPower(u32),

    /// Magical raw power
    MagicalRawPower(u32),

    /// Add % golds for each success mission
    Gold(u32),

    /// increase the chance to earn a second loot in % for each success mission
    LuckyLoot(u8),

    /// Add % experience for each success mission
    Experience(u32),

    #[deprecated]
    /// Increate the recruit's equipment (all of them) stats by %
    Reinforcement(u32),

    #[deprecated]
    /// Increate the native recruit stats (all of them) by %
    NaturalGrowth(u32),

    /// Amount of golds it can be increased when a buyer is interested
    Collector(u32),

    /// Increase the native recruit defense
    NaturalRawDefense(u32),
}
