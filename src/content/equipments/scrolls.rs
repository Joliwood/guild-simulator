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
                id: 1,
                image_atlas_index: 0,
                name: "Scroll of Wisdom".to_string(),
                price: 5,
                power: 3,
            },
            ScrollsEnum::ScrollOfPower => Scroll {
                id: 2,
                image_atlas_index: 1,
                name: "Scroll of Power".to_string(),
                price: 2,
                power: 5,
            },
            ScrollsEnum::ScrollOfEndurance => Scroll {
                id: 3,
                image_atlas_index: 2,
                name: "Scroll of Endurance".to_string(),
                price: 7,
                power: 5,
            },
            ScrollsEnum::ScrollOfSpeed => Scroll {
                id: 4,
                image_atlas_index: 3,
                name: "Scroll of Speed".to_string(),
                price: 4,
                power: 4,
            },
            ScrollsEnum::ScrollOfTheAncients => Scroll {
                id: 5,
                image_atlas_index: 3,
                name: "Scroll of the Ancients".to_string(),
                price: 10,
                power: 7,
            },
        }
    }
}
