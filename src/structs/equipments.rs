use crate::enums::ItemRaretyEnum;
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
                "{}\nPrice: {}\nStrength: {}\nIntelligence: {}\nEndurance: {}",
                weapon.name,
                weapon.price,
                weapon.strength.unwrap_or(0),
                weapon.intelligence.unwrap_or(0),
                weapon.endurance.unwrap_or(0)
            ),
            ItemEnum::Armor(armor) => format!(
                "{}\nPrice: {}\nStrength: {}\nIntelligence: {}\nEndurance: {}",
                armor.name,
                armor.price,
                armor.strength.unwrap_or(0),
                armor.intelligence.unwrap_or(0),
                armor.endurance.unwrap_or(0)
            ),
            ItemEnum::Scroll(scroll, _) => format!(
                "{}\nPrice: {}\nStrength: {}\nIntelligence: {}\nEndurance: {}",
                scroll.name,
                scroll.price,
                scroll.strength.unwrap_or(0),
                scroll.intelligence.unwrap_or(0),
                scroll.endurance.unwrap_or(0)
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Weapon {
    pub endurance: Option<u32>,
    pub id: u16,
    pub image_atlas_index: u16,
    pub intelligence: Option<u32>,
    pub name: String,
    pub price: u16,
    pub strength: Option<u32>,
    pub rarety: ItemRaretyEnum,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Armor {
    pub endurance: Option<u32>,
    pub id: u16,
    pub image_atlas_index: u16,
    pub intelligence: Option<u32>,
    pub name: String,
    pub price: u16,
    pub strength: Option<u32>,
    pub rarety: ItemRaretyEnum,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Scroll {
    pub endurance: Option<u32>,
    pub id: u16,
    pub image_atlas_index: u16,
    pub intelligence: Option<u32>,
    pub name: String,
    pub price: u16,
    pub strength: Option<u32>,
}
