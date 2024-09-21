#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Scroll(Scroll, u32),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Weapon {
    pub id: u32,
    pub name: String,
    pub strength: Option<u32>,
    pub endurance: Option<u32>,
    pub intelligence: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Armor {
    pub id: u32,
    pub name: String,
    pub strength: Option<u32>,
    pub endurance: Option<u32>,
    pub intelligence: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scroll {
    pub id: u32,
    pub name: String,
    pub strength: Option<u32>,
    pub endurance: Option<u32>,
    pub intelligence: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Weapons {
    pub items: Vec<Weapon>,
}
