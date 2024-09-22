#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Scroll(Scroll, u16),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Weapon {
    pub endurance: Option<u32>,
    pub id: u16,
    pub image_atlas_index: u16,
    pub intelligence: Option<u32>,
    pub name: String,
    pub price: u16,
    pub strength: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Armor {
    pub endurance: Option<u32>,
    pub id: u16,
    pub image_atlas_index: u16,
    pub intelligence: Option<u32>,
    pub name: String,
    pub price: u16,
    pub strength: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
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
