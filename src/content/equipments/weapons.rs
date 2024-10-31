#![allow(dead_code)]
use crate::{enums::ItemRaretyEnum, structs::equipments::Weapon};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WeaponsEnum {
    SwordOfValor,
    AxeOfFury,
    SpearOfDestiny,
    BowOfTheEagle,
    MaceOfTheThunder,
}

impl WeaponsEnum {
    /// Get a weapon by its enum variant
    pub fn get_weapon(&self) -> Weapon {
        match self {
            WeaponsEnum::SwordOfValor => Weapon {
                endurance: Some(5),
                id: 1,
                image_atlas_index: 0,
                intelligence: None,
                name: "Sword of Valor".to_string(),
                price: 100,
                rarety: ItemRaretyEnum::Common,
                strength: Some(20),
            },
            WeaponsEnum::AxeOfFury => Weapon {
                endurance: None,
                id: 2,
                image_atlas_index: 1,
                intelligence: None,
                name: "Axe of Fury".to_string(),
                price: 25,
                rarety: ItemRaretyEnum::Common,
                strength: Some(25),
            },
            WeaponsEnum::SpearOfDestiny => Weapon {
                endurance: None,
                id: 3,
                image_atlas_index: 2,
                intelligence: Some(5),
                name: "Spear of Destiny".to_string(),
                price: 50,
                rarety: ItemRaretyEnum::Common,
                strength: Some(15),
            },
            WeaponsEnum::BowOfTheEagle => Weapon {
                endurance: None,
                id: 4,
                image_atlas_index: 3,
                intelligence: Some(8),
                name: "Bow of the Eagle".to_string(),
                price: 75,
                rarety: ItemRaretyEnum::Common,
                strength: Some(18),
            },
            WeaponsEnum::MaceOfTheThunder => Weapon {
                endurance: None,
                id: 5,
                image_atlas_index: 4,
                intelligence: None,
                name: "Mace of the Thunder".to_string(),
                price: 125,
                rarety: ItemRaretyEnum::UnCommon,
                strength: Some(22),
            },
        }
    }
}
