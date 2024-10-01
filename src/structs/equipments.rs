#![allow(dead_code)]
use bevy::prelude::Component;
use serde::Deserialize;

use crate::enums::ItemRaretyEnum;

#[derive(Debug, Clone, Deserialize, Component, PartialEq, Eq, Hash)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Scroll(Scroll, u16),
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

#[derive(Debug, Deserialize)]
pub struct Weapons {
    pub items: Vec<Weapon>,
}

#[derive(Debug, Deserialize)]
pub struct Scrolls {
    pub items: Vec<Scroll>,
}

#[derive(Debug, Deserialize)]
pub struct Armors {
    pub items: Vec<Armor>,
}
