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
                experience: 0,
                level: 1,
                name: "Ennemy 1".to_string(),
                power: 17,
            },
            unlocked: true,
            description: "A basic camp, we think we could find some resources here. We need to send a recruit to check it out."
            .to_string(),
            // WIP - Rechange after tests to 5
            golds: 100,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::MagicToothpick),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Armor(ArmorsEnum::MakeshiftVest),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfExperienceI),
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
                experience: 0,
                level: 2,
                name: "Ennemy 2".to_string(),
                power: 27,
            },
            unlocked: true,
            description: "More extended camp, we think we could find some resources here. We need to send a recruit to check it out.".to_string(),
            golds: 10,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::WalkingStick),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Armor(ArmorsEnum::UsedLeatherToga),
                        percent: 10,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheMiserI),
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
                experience: 0,
                level: 3,
                name: "Ennemy 3".to_string(),
                power: 50,
            },
            unlocked: false,
            description: "A very extended camp, the final one of the tuto".to_string(),
            golds: 30,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfRawPowerI),
                        percent: 10,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfReinforcementI),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheMiserI),
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
                experience: 0,
                level: 1,
                name: "Ennemy 4".to_string(),
                power: 25,
            },
            unlocked: false,
            description: "A basic camp, we think we could find some resources here. We need to send a recruit to check it out."
            .to_string(),
            golds: 12,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::BowWithoutString),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Armor(ArmorsEnum::ApprenticeCoat),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::UnsharpDagger),
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
                experience: 0,
                level: 2,
                name: "Ennemy 5".to_string(),
                power: 37,
            },
            unlocked: false,
            description: "More extended camp, we think we could find some resources here. We need to send a recruit to check it out.".to_string(),
            golds: 15,
            loots: Loots(
                vec![
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::WoodenSword),
                        percent: 10,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Weapon(WeaponsEnum::LumberjackAxe),
                        percent: 50,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfTheResearcherI),
                        percent: 10,
                    },
                    ItemLoot {
                        item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfGaladornFailedPower),
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
                experience: 0,
                level: 3,
                name: "Ennemy 6".to_string(),
                power: 50,
            },
            unlocked: false,
            description: "A very extended camp, the final one of the tuto".to_string(),
            golds: 50,
            loots: Loots(vec![
                ItemLoot {
                    item: ItemLootEnum::Scroll(ScrollsEnum::ScrollOfNaturalGrowthI),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Armor(ArmorsEnum::RecycledMagicianRobe),
                    percent: 50,
                },
                ItemLoot {
                    item: ItemLootEnum::Armor(ArmorsEnum::LeatherTunic),
                    percent: 50,
                },
            ]),
            unlock_mission_ids: vec![],
        },
    ])
}
