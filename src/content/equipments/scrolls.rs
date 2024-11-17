#![allow(dead_code)]
use crate::structs::equipments::{BonusEnum, Scroll};

/// Parchemin de puissance brute I -> + 10 puissance
/// Parchemin de l'avare I -> + 5% de golds rapportées par mission réussite
/// ? Pas sûr du nom du chercheur -> Parchemin du chercheur I -> + 5% de chance d'obtenir un second loot par mission réussite
/// Parchemin d'expérience I -> + 5% d'expérience gagnée par mission réussite
/// Parchemin de renforcement I -> +5% de puissance sur les équipements
/// Parchemin d'acroissement naturel I -> +5% sur la puissance native de la recrue
/// Parchemin de puissance raté de Galadorn -> +1 puissance (mais a été signé par le célèbre archimage Galadorn, il a une forte valeur pour tous les fans de magie)
///
/// Parchemin d'accroissement musculaire = +5 puissance physique
/// Parchemin de concentration arcanique = 5% puissance magique
/// Parchemin de vivacité : Augmente la défense 3 + puissance physique de 2
/// Parchemin d'irrigation arcanique + 5 puissance magique
/// Parchemin de défense naturelle brute +5 défense

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScrollsEnum {
    /// ## Integrated in :
    /// - Mission 3
    ScrollOfPhysicalRawPowerI,

    /// ## Integrated in :
    /// TODO - Ajouter à la première mission à la place de la mission 2 en dessous the miser
    ScrollOfMagicalRawPowerI,

    // TODO - Déplacer sur la dernière ou avant dernière mission pour farmer les premières, c'est plus logique
    /// ## Integrated in :
    /// - Mission 2
    /// - Mission 3
    ScrollOfTheMiserI,

    /// ## Integrated in :
    /// - Mission 5
    ScrollOfTheResearcherI,

    /// ## Integrated in :
    /// - Mission 1
    ScrollOfExperienceI,

    /// ## Integrated in :
    /// - Mission 3
    ScrollOfReinforcementI,

    /// ## Integrated in :
    /// - Mission 6
    ScrollOfNaturalGrowthI,

    /// ## Integrated in :
    /// - Mission 5
    ScrollOfGaladornFailedPower,

    /// ## Integrated in :
    /// TODO
    ScrollOfRawNaturalDefense,
}

impl ScrollsEnum {
    /// Get a scroll by its enum variant
    pub fn get_scroll(&self) -> Scroll {
        match self {
            ScrollsEnum::ScrollOfPhysicalRawPowerI => Scroll {
                id: 1,
                image_atlas_index: 0,
                name: "Scroll of Wisdom I".to_string(),
                price: 5,
                bonus: vec![BonusEnum::PhysicalRawPower(10)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfMagicalRawPowerI => Scroll {
                id: 1,
                image_atlas_index: 0,
                name: "Scroll of Wisdom I".to_string(),
                price: 5,
                bonus: vec![BonusEnum::PhysicalRawPower(10)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfTheMiserI => Scroll {
                id: 2,
                image_atlas_index: 1,
                name: "Scroll of the Miser I".to_string(),
                price: 5,
                bonus: vec![BonusEnum::Gold(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfTheResearcherI => Scroll {
                id: 3,
                image_atlas_index: 2,
                name: "Scroll of the Researcher I".to_string(),
                price: 5,
                bonus: vec![BonusEnum::LuckyLoot(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfExperienceI => Scroll {
                id: 4,
                image_atlas_index: 3,
                name: "Scroll of Experience I".to_string(),
                price: 5,
                bonus: vec![BonusEnum::Experience(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfReinforcementI => Scroll {
                id: 5,
                image_atlas_index: 4,
                name: "Scroll of Reinforcement I".to_string(),
                price: 5,
                bonus: vec![BonusEnum::Reinforcement(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfNaturalGrowthI => Scroll {
                id: 6,
                image_atlas_index: 5,
                name: "Scroll of Natural Growth I".to_string(),
                price: 5,
                bonus: vec![BonusEnum::NaturalGrowth(5)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfGaladornFailedPower => Scroll {
                id: 7,
                image_atlas_index: 6,
                name: "Scroll of Galadorn Failed Power".to_string(),
                price: 25,
                bonus: vec![BonusEnum::MagicalRawPower(1), BonusEnum::Collector(65)],
                ..Default::default()
            },
            ScrollsEnum::ScrollOfRawNaturalDefense => Scroll {
                id: 8,
                image_atlas_index: 6,
                name: "Scroll of Raw Natural Defense".to_string(),
                price: 7,
                bonus: vec![BonusEnum::NaturalRawDefense(1)],
                ..Default::default()
            },
        }
    }
}
