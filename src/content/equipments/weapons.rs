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
                id: 1,
                image_atlas_index: 0,
                name: "Sword of Valor".to_string(),
                price: 10,
                rarety: ItemRaretyEnum::Common,
                power: 12,
            },
            WeaponsEnum::AxeOfFury => Weapon {
                id: 2,
                image_atlas_index: 1,
                name: "Axe of Fury".to_string(),
                price: 2,
                rarety: ItemRaretyEnum::Common,
                power: 5,
            },
            WeaponsEnum::SpearOfDestiny => Weapon {
                id: 3,
                image_atlas_index: 2,
                name: "Spear of Destiny".to_string(),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                power: 7,
            },
            WeaponsEnum::BowOfTheEagle => Weapon {
                id: 4,
                image_atlas_index: 3,
                name: "Bow of the Eagle".to_string(),
                price: 12,
                rarety: ItemRaretyEnum::Common,
                power: 11,
            },
            WeaponsEnum::MaceOfTheThunder => Weapon {
                id: 5,
                image_atlas_index: 4,
                name: "Mace of the Thunder".to_string(),
                price: 12,
                rarety: ItemRaretyEnum::UnCommon,
                power: 6,
            },
        }
    }
}
