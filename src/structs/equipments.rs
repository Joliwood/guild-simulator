use bevy::{math::UVec2, prelude::Component, sprite::TextureAtlasLayout};
use serde::Deserialize;

use crate::enums::ItemRaretyEnum;

#[derive(Debug, Clone, Deserialize, Component, PartialEq, Eq, Hash)]
pub enum ItemEnum {
    Weapon(Weapon),
    Armor(Armor),
    Scroll(Scroll, u16),
}

impl ItemEnum {
    pub fn get_atlas_index(&self) -> u16 {
        match self {
            ItemEnum::Weapon(weapon) => weapon.image_atlas_index,
            ItemEnum::Armor(armor) => armor.image_atlas_index,
            ItemEnum::Scroll(scroll, _) => scroll.image_atlas_index,
        }
    }

    pub fn get_item_layout(&self) -> TextureAtlasLayout {
        match self {
            ItemEnum::Weapon(_) => TextureAtlasLayout::from_grid(
                UVec2::new(2900, 400),
                6,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            ),
            ItemEnum::Armor(_) => TextureAtlasLayout::from_grid(
                UVec2::new(1600, 400),
                4,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            ),
            ItemEnum::Scroll(_, _) => TextureAtlasLayout::from_grid(
                UVec2::new(4320, 1080),
                4,
                1,
                Some(UVec2::new(0, 0)),
                Some(UVec2::new(0, 0)),
            ),
        }
    }

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
