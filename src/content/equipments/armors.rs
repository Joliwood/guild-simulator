#![allow(dead_code)]
use crate::{enums::ItemRaretyEnum, structs::equipments::Armor};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ArmorsEnum {
    ShieldOfCourage,
    HelmetOfTheGuardian,
    BreastplateOfTheDragon,
    GauntletsOfPower,
    // BootsOfSwiftness,
}

impl ArmorsEnum {
    /// Get an armor by its enum variant
    pub fn get_armor(&self) -> Armor {
        match self {
            ArmorsEnum::ShieldOfCourage => Armor {
                endurance: Some(30),
                id: 1,
                image_atlas_index: 0,
                intelligence: None,
                name: "Shield of Courage".to_string(),
                price: 50,
                rarety: ItemRaretyEnum::Common,
                strength: None,
            },
            ArmorsEnum::HelmetOfTheGuardian => Armor {
                endurance: Some(20),
                id: 2,
                image_atlas_index: 1,
                intelligence: Some(5),
                name: "Helmet of the Guardian".to_string(),
                price: 30,
                rarety: ItemRaretyEnum::Common,
                strength: None,
            },
            ArmorsEnum::BreastplateOfTheDragon => Armor {
                endurance: Some(35),
                id: 3,
                image_atlas_index: 2,
                intelligence: None,
                name: "Breastplate of the Dragon".to_string(),
                price: 75,
                rarety: ItemRaretyEnum::UnCommon,
                strength: Some(10),
            },
            ArmorsEnum::GauntletsOfPower => Armor {
                endurance: Some(15),
                id: 4,
                image_atlas_index: 3,
                intelligence: None,
                name: "Gauntlets of Power".to_string(),
                price: 40,
                rarety: ItemRaretyEnum::UnCommon,
                strength: Some(12),
            },
            // ArmorsEnum::BootsOfSwiftness => Armor {
            //     endurance: Some(18),
            //     id: 5,
            //     image_atlas_index: 4,
            //     intelligence: Some(3),
            //     name: "Boots of Swiftness".to_string(),
            //     price: 25,
            //     rarety: ItemRaretyEnum::Rare,
            //     strength: None,
            // },
        }
    }
}
