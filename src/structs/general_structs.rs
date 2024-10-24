#![allow(dead_code)]
use super::equipments::{Armor, Scroll, Weapon};
use crate::data::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum};
use bevy::prelude::{Component, Resource};
// ! For workflow with ron files
// use ron::de::from_str;

#[derive(Resource)]
pub struct MissionNotificationsNumber(pub u8);

#[derive(Component, Resource)]
pub struct MissionModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct MissionReportsModalVisible(pub bool);

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
    pub endurance: u16,
    pub experience: u32,
    pub intelligence: u16,
    pub level: u8,
    pub name: String,
    pub strength: u16,
    pub image_atlas_index: u16,
}

// ! Version with Ron + serde
// ! Not working currently with builds
// pub fn load_weapon_by_id(id: u16) -> Option<Weapon> {
//     let weapons_data = fs::read_to_string("src/data/equipments/weapons.ron")
//         .expect("Failed to read the RON file.");

//     let weapons: Weapons = from_str(&weapons_data).expect("Failed to deserialize RON data.");

//     if let Some(weapon) = weapons.items.iter().find(|weapon| weapon.id == id) {
//         info!("Weapon with id = {}: {:?}", id, weapon);
//         return Some(weapon.clone());
//     } else {
//         info!("Weapon with id = {} not found.", id);
//         return None;
//     }
// }

pub fn load_weapon(weapon: WeaponsEnum) -> Weapon {
    return WeaponsEnum::get_weapon(&weapon);
}

// ! Version with Ron + serde
// ! Not working currently with builds
// pub fn load_scroll_by_id(id: u16) -> Option<Scroll> {
//     let scrolls_data = fs::read_to_string("src/data/equipments/scrolls.ron")
//         .expect("Failed to read the RON file.");

//     let scrolls: Scrolls = from_str(&scrolls_data).expect("Failed to deserialize RON data.");

//     if let Some(scroll) = scrolls.items.iter().find(|scroll| scroll.id == id) {
//         info!("Scroll with id = {}: {:?}", id, scroll);
//         return Some(scroll.clone());
//     } else {
//         info!("Scroll with id = {} not found.", id);
//         return None;
//     }
// }

pub fn load_scroll(scroll: ScrollsEnum) -> Scroll {
    return ScrollsEnum::get_scroll(&scroll);
}

// ! Version with Ron + serde
// ! Not working currently with builds
// pub fn load_armor_by_id(id: u16) -> Option<Armor> {
//     let armors_data =
//         fs::read_to_string("src/data/equipments/armors.ron").expect("Failed to read the RON file.");

//     let armors: Armors = from_str(&armors_data).expect("Failed to deserialize RON data.");

//     if let Some(armor) = armors.items.iter().find(|armor| armor.id == id) {
//         info!("Armor with id = {}: {:?}", id, armor);
//         return Some(armor.clone());
//     } else {
//         info!("Armor with id = {} not found.", id);
//         return None;
//     }
// }

pub fn load_armor(armor: ArmorsEnum) -> Armor {
    return ArmorsEnum::get_armor(&armor);
}
