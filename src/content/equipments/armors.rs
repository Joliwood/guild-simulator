#![allow(dead_code)]
use crate::{enums::ItemRaretyEnum, structs::equipments::Armor};

/// Traductions fr :
/// Toge en cuir usagée + 4 puissance
/// Manteau d'apprenti + 6 puissance
/// Robe de Magicien Recyclée -> +4 puissance + optimisé pour les classes magiques + 5 puissance
/// Tunique en cuir + 12 puissance
/// Gilet de fortune + 5 puissance
/// Toge de Voyage + 7 puissance

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ArmorsEnum {
    /// ## Integrated in :
    /// - Mission 2
    UsedLeatherToga,

    /// ## Integrated in :
    /// - Mission 4
    ApprenticeCoat,

    /// ## Integrated in :
    /// - Mission 6
    RecycledMagicianRobe,

    /// ## Integrated in :
    /// - Mission 6
    LeatherTunic,

    /// ## Integrated in :
    /// - Mission 1
    MakeshiftVest,

    /// ## Integrated in :
    /// - Daily discussion 10
    TravelToga,
}

impl ArmorsEnum {
    /// Get an armor by its enum variant
    pub fn get_armor(&self) -> Armor {
        match self {
            ArmorsEnum::UsedLeatherToga => Armor {
                id: 1,
                image_atlas_index: 0,
                name: "Used leather toga".to_string(),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                power: 4,
            },
            ArmorsEnum::ApprenticeCoat => Armor {
                id: 2,
                image_atlas_index: 1,
                name: "Apprentice coat".to_string(),
                price: 7,
                rarety: ItemRaretyEnum::Common,
                power: 6,
            },
            ArmorsEnum::RecycledMagicianRobe => Armor {
                id: 3,
                image_atlas_index: 2,
                name: "Recycled magician robe".to_string(),
                price: 10,
                rarety: ItemRaretyEnum::Common,
                // TODO - Add a status optimized for magic classes + 5 power
                power: 4,
            },
            ArmorsEnum::LeatherTunic => Armor {
                id: 4,
                image_atlas_index: 3,
                name: "Leather tunic".to_string(),
                price: 12,
                rarety: ItemRaretyEnum::Common,
                power: 12,
            },
            ArmorsEnum::MakeshiftVest => Armor {
                id: 5,
                image_atlas_index: 4,
                name: "Makeshift vest".to_string(),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                power: 5,
            },
            ArmorsEnum::TravelToga => Armor {
                id: 6,
                image_atlas_index: 5,
                name: "Travel toga".to_string(),
                price: 7,
                rarety: ItemRaretyEnum::Common,
                power: 7,
            },
        }
    }
}
