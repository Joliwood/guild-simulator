#![allow(dead_code)]

use super::equipments::{Armor, Scroll, Weapon};
use crate::content::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum};
use bevy::prelude::{Component, Resource};
// ! For workflow with ron files
// use ron::de::from_str;

#[derive(Resource)]
pub struct MissionNotificationsNumber(pub u8);

#[derive(Component, Resource)]
pub struct MissionModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct MissionReportsModalVisible(pub bool);

#[derive(Component, Resource)]
pub struct DailyEventsModalVisible(pub bool);

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
    pub experience: u32,
    pub level: u8,
    pub name: String,
    pub physical_power: u32,
    pub magical_power: u32,
    pub defense: u32,
    pub image_atlas_index: u16,
}

impl Ennemy {
    // Function to return the % part of the fight that is physical
    pub fn get_physical_fight_part(&self) -> f32 {
        let calcul: f32 = self.physical_power as f32
            / (self.physical_power as f32 + self.magical_power as f32)
            * 100.;
        return calcul;
    }

    // Function to return the % part of the fight that is magical
    pub fn get_magical_fight_part(&self) -> f32 {
        let calcul: f32 = self.magical_power as f32
            / (self.physical_power as f32 + self.magical_power as f32)
            * 100.;
        return calcul;
    }
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

#[derive(Default, Resource)]
pub struct DayTime {
    pub current_time: f32,
    pub elapsed_time: f32,
    pub second_count: u32,
}

impl DayTime {
    fn get_hours_minutes(&self) -> (u32, u32) {
        let total_minutes = self.current_time as u32;
        let hours = 8 + total_minutes / 60;
        let minutes = total_minutes % 60;
        (hours, minutes)
    }

    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.elapsed_time = 0.0;
        self.second_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_physical_fight_part() {
        // Basic example
        assert_eq!(
            Ennemy {
                experience: 0,
                level: 0,
                name: "".to_string(),
                physical_power: 82,
                magical_power: 75,
                defense: 56,
                image_atlas_index: 0,
            }
            .get_physical_fight_part(),
            52.229298
        );

        // Should work with 0 physical power
        assert_eq!(
            Ennemy {
                experience: 0,
                level: 0,
                name: "".to_string(),
                physical_power: 0,
                magical_power: 75,
                defense: 56,
                image_atlas_index: 0,
            }
            .get_physical_fight_part(),
            0.
        );

        // Should return 100 if magical power is 0
        assert_eq!(
            Ennemy {
                experience: 0,
                level: 0,
                name: "".to_string(),
                physical_power: 82,
                magical_power: 0,
                defense: 56,
                image_atlas_index: 0,
            }
            .get_physical_fight_part(),
            100.
        );
    }

    #[test]
    fn get_magical_fight_part() {
        // Basic example
        assert_eq!(
            Ennemy {
                experience: 0,
                level: 0,
                name: "".to_string(),
                physical_power: 82,
                magical_power: 75,
                defense: 56,
                image_atlas_index: 0,
            }
            .get_magical_fight_part(),
            47.7707
        );

        // Should work with 0 magical power
        assert_eq!(
            Ennemy {
                experience: 0,
                level: 0,
                name: "".to_string(),
                physical_power: 82,
                magical_power: 0,
                defense: 56,
                image_atlas_index: 0,
            }
            .get_magical_fight_part(),
            0.
        );

        // Should return 100 if physical power is 0
        assert_eq!(
            Ennemy {
                experience: 0,
                level: 0,
                name: "".to_string(),
                physical_power: 0,
                magical_power: 75,
                defense: 56,
                image_atlas_index: 0,
            }
            .get_magical_fight_part(),
            100.
        );
    }
}
