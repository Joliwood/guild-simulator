use crate::structs::equipments::Scroll;

#[derive(Debug, Clone)]
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
                intelligence: Some(10),
                name: "Scroll of Wisdom".to_string(),
                price: 50,
                strength: None,
            },
            ScrollsEnum::ScrollOfPower => Scroll {
                endurance: None,
                id: 2,
                image_atlas_index: 1,
                intelligence: Some(5),
                name: "Scroll of Power".to_string(),
                price: 25,
                strength: Some(15),
            },
            ScrollsEnum::ScrollOfEndurance => Scroll {
                endurance: Some(8),
                id: 3,
                image_atlas_index: 2,
                intelligence: None,
                name: "Scroll of Endurance".to_string(),
                price: 75,
                strength: None,
            },
            ScrollsEnum::ScrollOfSpeed => Scroll {
                endurance: Some(12),
                id: 4,
                image_atlas_index: 3,
                intelligence: Some(8),
                name: "Scroll of Speed".to_string(),
                price: 40,
                strength: None,
            },
            ScrollsEnum::ScrollOfTheAncients => Scroll {
                endurance: None,
                id: 5,
                image_atlas_index: 4,
                intelligence: Some(20),
                name: "Scroll of the Ancients".to_string(),
                price: 100,
                strength: Some(10),
            },
        }
    }
}
