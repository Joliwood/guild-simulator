#![allow(dead_code)]
use crate::{enums::ItemRaretyEnum, structs::equipments::Weapon};

/// Traductions fr :
/// épée en bois + 10 puissance
/// arc sans corde + 3 puissance -> avec un status optimisé pour les chasseurs + 5 puissance + ajouter un texte, "peut être qu'un chasseur aurait des cordes de disponible pour le rendre efficace..."
/// hache de bucheron fendue + 6 puissance
/// cure-dent magique + 2 puissance -> avec un status optimisé pour les classes magiques + 5 puissance
/// dague émoussée + 4 puissance
/// baton de marche + 5 puissance

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WeaponsEnum {
    /// ## Integrated in :
    /// - Mission 5
    /// - Daily discussion 10
    WoodenSword,

    /// ## Integrated in :
    /// - Mission 4
    BowWithoutString,

    /// ## Integrated in :
    /// - Mission 5
    LumberjackAxe,

    /// ## Integrated in :
    /// - Mission 1
    MagicToothpick,

    /// ## Integrated in :
    /// - Mission 4
    UnsharpDagger,

    /// ## Integrated in :
    /// - Mission 2
    WalkingStick,
}

impl WeaponsEnum {
    /// Get a weapon by its enum variant
    pub fn get_weapon(&self) -> Weapon {
        match self {
            WeaponsEnum::WoodenSword => Weapon {
                id: 1,
                image_atlas_index: 0,
                name: "Wooden sword".to_string(),
                price: 10,
                rarety: ItemRaretyEnum::Common,
                power: 10,
            },
            WeaponsEnum::BowWithoutString => Weapon {
                id: 2,
                image_atlas_index: 1,
                name: "Bow without string".to_string(),
                price: 2,
                rarety: ItemRaretyEnum::Common,
                // TODO - Add a status optimized for hunters + 5 power + add a text, "maybe a hunter would have strings available to make it effective..."
                power: 3,
            },
            WeaponsEnum::LumberjackAxe => Weapon {
                id: 3,
                image_atlas_index: 2,
                name: "Lumberjack axe".to_string(),
                price: 5,
                rarety: ItemRaretyEnum::Common,
                power: 6,
            },
            WeaponsEnum::MagicToothpick => Weapon {
                id: 4,
                image_atlas_index: 3,
                name: "Magic toothpick".to_string(),
                price: 2,
                rarety: ItemRaretyEnum::Common,
                // TODO - Add a status optimized for magic classes + 5 power
                power: 2,
            },
            WeaponsEnum::UnsharpDagger => Weapon {
                id: 5,
                image_atlas_index: 4,
                name: "Unsharp dagger".to_string(),
                price: 4,
                rarety: ItemRaretyEnum::Common,
                power: 4,
            },
            WeaponsEnum::WalkingStick => Weapon {
                id: 6,
                image_atlas_index: 5,
                name: "Walking stick".to_string(),
                price: 3,
                rarety: ItemRaretyEnum::Common,
                power: 5,
            },
        }
    }
}
