#![allow(dead_code)]
use crate::{enums::ItemRaretyEnum, structs::equipments::Armor};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ArmorsEnum {
    ShieldOfCourage,
    HelmetOfTheGuardian,
    BreastplateOfTheDragon,
    GauntletsOfPower,
    // ⬇ WIP equipment V1 ⬇
    // WoodenSword,
    // StringlessBox,
    // LumberjackAxe,
    // MagicToothpick,
    // UnsharpDagger,
}

impl ArmorsEnum {
    /// Get an armor by its enum variant
    pub fn get_armor(&self) -> Armor {
        match self {
            ArmorsEnum::ShieldOfCourage => Armor {
                endurance: Some(10),
                id: 1,
                image_atlas_index: 0,
                intelligence: None,
                name: "Shield of Courage".to_string(),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                strength: None,
            },
            ArmorsEnum::HelmetOfTheGuardian => Armor {
                endurance: Some(2),
                id: 2,
                image_atlas_index: 1,
                intelligence: Some(10),
                name: "Helmet of the Guardian".to_string(),
                price: 3,
                rarety: ItemRaretyEnum::Common,
                strength: None,
            },
            ArmorsEnum::BreastplateOfTheDragon => Armor {
                endurance: Some(6),
                id: 3,
                image_atlas_index: 2,
                intelligence: None,
                name: "Breastplate of the Dragon".to_string(),
                price: 7,
                rarety: ItemRaretyEnum::UnCommon,
                strength: Some(8),
            },
            ArmorsEnum::GauntletsOfPower => Armor {
                endurance: Some(3),
                id: 4,
                image_atlas_index: 3,
                intelligence: None,
                name: "Gauntlets of Power".to_string(),
                price: 40,
                rarety: ItemRaretyEnum::UnCommon,
                strength: Some(7),
            },
        }
    }
}
