#![allow(dead_code)]

use super::equipments::{armors::ArmorsEnum, scrolls::ScrollsEnum, weapons::WeaponsEnum};
use crate::structs::{
    general_structs::Ennemy,
    missions::{ItemLoot, ItemLootEnum, Loots, Mission, Missions},
};

pub fn generate_all_missions() -> Missions {
    Missions(vec![
        Mission {
            days_left: None,
            days: 1,
            id: 1,
            level: 1,
            name: "Mission 1".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 5,
                endurance: 5,
                experience: 0,
                intelligence: 5,
                level: 1,
                name: "Ennemy 1".to_string(),
                strength: 7,
            },
            unlocked: true,
            description: "A basic camp, we think we could find some resources here. We need to send a recruit to check it out."
            .to_string(),
            golds: 5,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::SwordOfValor),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Armor(ArmorsEnum::GauntletsOfPower),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfWisdom),
                        percent: 1,
                    },
                ]
            ),
            unlock_mission_ids: vec![4, 5],
        },
        Mission {
            days_left: None,
            days: 1,
            id: 2,
            level: 2,
            name: "Mission 2".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 4,
                endurance: 15,
                experience: 0,
                intelligence: 7,
                level: 2,
                name: "Ennemy 2".to_string(),
                strength: 5,
            },
            unlocked: true,
            description: "More extended camp, we think we could find some resources here. We need to send a recruit to check it out.".to_string(),
            golds: 10,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::BowOfTheEagle),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Armor(ArmorsEnum::BreastplateOfTheDragon),
                        percent: 10,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfEndurance),
                        percent: 10,
                    },
                ],
            ),
            unlock_mission_ids: vec![3],
        },
        Mission {
            days_left: Some(1),
            days: 2,
            id: 3,
            level: 3,
            name: "Mission 3".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 3,
                endurance: 20,
                experience: 0,
                intelligence: 10,
                level: 3,
                name: "Ennemy 3".to_string(),
                strength: 20,
            },
            unlocked: false,
            description: "A very extended camp, the final one of the tuto".to_string(),
            golds: 30,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfSpeed),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfPower),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheAncients),
                        percent: 50,
                    },
                ],
            ),
            unlock_mission_ids: vec![],
        },
        Mission {
            days_left: None,
            days: 1,
            id: 4,
            level: 1,
            name: "Mission 4".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 5,
                endurance: 10,
                experience: 0,
                intelligence: 5,
                level: 1,
                name: "Ennemy 4".to_string(),
                strength: 10,
            },
            unlocked: false,
            description: "A basic camp, we think we could find some resources here. We need to send a recruit to check it out."
            .to_string(),
            golds: 12,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::SwordOfValor),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Armor(ArmorsEnum::GauntletsOfPower),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfWisdom),
                        percent: 50,
                    },
                ]
            ),
            unlock_mission_ids: vec![5],
        },
        Mission {
            days_left: None,
            days: 1,
            id: 5,
            level: 2,
            name: "Mission 5".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 4,
                endurance: 15,
                experience: 0,
                intelligence: 7,
                level: 2,
                name: "Ennemy 5".to_string(),
                strength: 15,
            },
            unlocked: false,
            description: "More extended camp, we think we could find some resources here. We need to send a recruit to check it out.".to_string(),
            golds: 15,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::BowOfTheEagle),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Armor(ArmorsEnum::BreastplateOfTheDragon),
                        percent: 10,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfWisdom),
                        percent: 10,
                    },
                ],
            ),
            unlock_mission_ids: vec![6],
        },
        Mission {
            days_left: Some(1),
            days: 2,
            id: 6,
            level: 3,
            name: "Mission 6".to_string(),
            percent_of_victory: None,
            recruit_send: None,
            ennemy: Ennemy {
                image_atlas_index: 3,
                endurance: 20,
                experience: 0,
                intelligence: 10,
                level: 3,
                name: "Ennemy 6".to_string(),
                strength: 20,
            },
            unlocked: false,
            description: "A very extended camp, the final one of the tuto".to_string(),
            golds: 50,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfSpeed),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfPower),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheAncients),
                    percent: 50,
                },
            ]),
            unlock_mission_ids: vec![],
        },
    ])
}
