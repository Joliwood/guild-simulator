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
                "{}\nPrice: {}\nPower: {}",
                weapon.name, weapon.price, weapon.power,
            ),
            ItemEnum::Armor(armor) => format!(
                "{}\nPrice: {}\nPower: {}",
                armor.name, armor.price, armor.power
            ),
            ItemEnum::Scroll(scroll, _) => format!(
                "{}\nPrice: {}\nPower: {}",
                scroll.name, scroll.price, scroll.power
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Weapon {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub power: u32,
    pub rarety: ItemRaretyEnum,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Armor {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub rarety: ItemRaretyEnum,
    pub power: u32,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub struct Scroll {
    pub id: u16,
    pub image_atlas_index: u16,
    pub name: String,
    pub price: u16,
    pub power: u32,
}
