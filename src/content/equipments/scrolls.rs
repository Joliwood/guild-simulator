#![allow(dead_code)]
use crate::structs::equipments::Scroll;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScrollsEnum {
    ScrollOfWisdom,
    ScrollOfPower,
    ScrollOfEndurance,
    ScrollOfSpeed,
    ScrollOfTheAncients,
}

impl ScrollsEnum {
    /// Get a scroll by its enum variant
    pub fn get_scroll(&self) -> Scroll {
        match self {
            ScrollsEnum::ScrollOfWisdom => Scroll {
                endurance: None,
                id: 1,
                image_atlas_index: 0,
                intelligence: Some(3),
                name: "Scroll of Wisdom".to_string(),
                price: 5,
                strength: None,
            },
            ScrollsEnum::ScrollOfPower => Scroll {
                endurance: None,
                id: 2,
                image_atlas_index: 1,
                intelligence: Some(5),
                name: "Scroll of Power".to_string(),
                price: 2,
                strength: None,
            },
            ScrollsEnum::ScrollOfEndurance => Scroll {
                endurance: Some(5),
                id: 3,
                image_atlas_index: 2,
                intelligence: None,
                name: "Scroll of Endurance".to_string(),
                price: 7,
                strength: None,
            },
            ScrollsEnum::ScrollOfSpeed => Scroll {
                endurance: Some(2),
                id: 4,
                image_atlas_index: 3,
                intelligence: Some(2),
                name: "Scroll of Speed".to_string(),
                price: 4,
                strength: None,
            },
            ScrollsEnum::ScrollOfTheAncients => Scroll {
                endurance: None,
                id: 5,
                image_atlas_index: 3,
                intelligence: Some(2),
                name: "Scroll of the Ancients".to_string(),
                price: 10,
                strength: Some(5),
            },
        }
    }
}
