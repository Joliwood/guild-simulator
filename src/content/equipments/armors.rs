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
                id: 1,
                image_atlas_index: 0,
                name: "Shield of Courage".to_string(),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                power: 10,
            },
            ArmorsEnum::HelmetOfTheGuardian => Armor {
                id: 2,
                image_atlas_index: 1,
                name: "Helmet of the Guardian".to_string(),
                price: 3,
                rarety: ItemRaretyEnum::Common,
                power: 12,
            },
            ArmorsEnum::BreastplateOfTheDragon => Armor {
                id: 3,
                image_atlas_index: 2,
                name: "Breastplate of the Dragon".to_string(),
                price: 7,
                rarety: ItemRaretyEnum::UnCommon,
                power: 14,
            },
            ArmorsEnum::GauntletsOfPower => Armor {
                id: 4,
                image_atlas_index: 3,
                name: "Gauntlets of Power".to_string(),
                price: 40,
                rarety: ItemRaretyEnum::UnCommon,
                power: 10,
            },
        }
    }
}
