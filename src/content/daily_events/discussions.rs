use crate::{
    content::equipments::{armors::ArmorsEnum, weapons::WeaponsEnum},
    structs::{
        daily_events_folder::{
            daily_events::{Answer, DaySystem, ImpactAction},
            discussions::DailyDiscussion,
        },
        equipments::ItemEnum,
    },
};

// --- To update whenever the content is updated --- //
const MAX_DAILY_DISCUSSION_NUMBER: u16 = 10;

pub fn get_all_daily_discussions() -> Vec<DailyDiscussion> {
    (1..=MAX_DAILY_DISCUSSION_NUMBER)
        .map(|i| get_daily_discussion(&i))
        .collect()
}

pub fn get_daily_discussion(daily_discussion_index: &u16) -> DailyDiscussion {
    match daily_discussion_index {
        1 => DailyDiscussion {
            id: 1,
            title: "discussion1_title".to_string(),
            description: "discussion1_desc".to_string(),
            image_atlas_index: 0,
            apparition_chance: 25,
            answers: vec![
                Answer {
                    id: 1,
                    message: "discussion1_answer1_msg".to_string(),
                    reputation_impact: Some(1),
                    toxicity_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "discussion1_answer2_msg".to_string(),
                    reputation_impact: Some(-2),
                    toxicity_impact: Some(3),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "discussion1_answer3_msg".to_string(),
                    reputation_impact: Some(-1),
                    toxicity_impact: Some(1),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 7,
                max_day: None,
                min_day: 1,
            },
        },
        2 => DailyDiscussion {
            id: 2,
            title: "discussion2_title".to_string(),
            description: "discussion2_desc".to_string(),
            image_atlas_index: 1,
            apparition_chance: 35,
            answers: vec![
                Answer {
                    id: 1,
                    message: "discussion2_answer1_msg".to_string(),
                    gold_impact: Some(-10),
                    toxicity_impact: Some(-3),
                    reputation_impact: Some(3),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "discussion2_answer2_msg".to_string(),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "discussion2_answer3_msg".to_string(),
                    toxicity_impact: Some(-1),
                    reputation_impact: Some(-2),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 3,
                max_day: None,
                min_day: 1,
            },
        },
        3 => DailyDiscussion {
            id: 3,
            title: "discussion3_title".to_string(),
            description: "discussion3_desc".to_string(),
            image_atlas_index: 2,
            apparition_chance: 15,
            answers: vec![
                Answer {
                    id: 1,
                    message: "discussion3_answer1_msg".to_string(),
                    reputation_impact: Some(1),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "discussion3_answer2_msg".to_string(),
                    reputation_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "discussion3_answer3_msg".to_string(),
                    gold_impact: Some(-5),
                    reputation_impact: Some(1),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 10,
                max_day: None,
                min_day: 2,
            },
        },
        4 => DailyDiscussion {
            id: 4,
            title: "discussion4_title".to_string(),
            description: "discussion4_desc".to_string(),
            image_atlas_index: 3,
            apparition_chance: 5,
            answers: vec![
                Answer {
                    id: 1,
                    message: "discussion4_answer1_msg".to_string(),
                    gold_impact: Some(-5),
                    reputation_impact: Some(2),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "discussion4_answer2_msg".to_string(),
                    reputation_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "discussion4_answer3_msg".to_string(),
                    toxicity_impact: Some(1),
                    reputation_impact: Some(-2),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 15,
                max_day: None,
                min_day: 5,
            },
        },
        5 => DailyDiscussion {
            id: 5,
            title: "discussion5_title".to_string(),
            description: "discussion5_desc".to_string(),
            image_atlas_index: 4,
            apparition_chance: 25,
            answers: vec![
                Answer {
                    id: 1,
                    message: "discussion5_answer1_msg".to_string(),
                    gold_impact: Some(-10),
                    toxicity_impact: Some(-2),
                    reputation_impact: Some(2),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "discussion5_answer2_msg".to_string(),
                    reputation_impact: Some(1),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "discussion5_answer3_msg".to_string(),
                    reputation_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 4,
                    message: "discussion5_answer4_msg".to_string(),
                    reputation_impact: Some(-3),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 5,
                max_day: None,
                min_day: 10,
            },
        },
        6 => DailyDiscussion {
            id: 6,
            title: "discussion6_title".to_string(),
            description: "discussion6_desc".to_string(),
            image_atlas_index: 5,
            apparition_chance: 20,
            answers: vec![
                Answer {
                    id: 1,
                    message: "discussion6_answer1_msg".to_string(),
                    gold_impact: Some(-5),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "discussion6_answer2_msg".to_string(),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "discussion6_answer3_msg".to_string(),
                    gold_impact: Some(-10),
                    ..Default::default()
                },
                Answer {
                    id: 4,
                    message: "discussion6_answer4_msg".to_string(),
                    toxicity_impact: Some(-1),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 2,
                max_day: None,
                min_day: 10,
            },
        },
        7 => DailyDiscussion {
            id: 7,
            title: "discussion7_title".to_string(),
            description: "discussion7_desc".to_string(),
            image_atlas_index: 6,
            apparition_chance: 100,
            answers: vec![Answer {
                gold_impact: Some(10),
                id: 1,
                message: "discussion7_answer1_msg".to_string(),
                ..Default::default()
            }],
            day_system: DaySystem {
                cooldown: 0,
                min_day: 1,
                max_day: Some(1),
            },
        },
        // 8 => DailyDiscussion {
        //     id: 8,
        //     title: "discussion8_title".to_string(),
        //     description: "discussion8_desc".to_string(),
        //     image_atlas_index: 7,
        //     apparition_chance: 100,
        //     answers: vec![Answer {
        //         gold_impact: Some(50),
        //         id: 1,
        //         message: "discussion8_answer1_msg".to_string(),
        //         ..Default::default()
        //     }],
        //     day_system: DaySystem {
        //         cooldown: 0,
        //         min_day: 1,
        //         max_day: Some(1),
        //     },
        // },
        8 => DailyDiscussion {
            id: 8,
            title: "discussion8_title".to_string(),
            description: "discussion8_desc".to_string(),
            image_atlas_index: 8,
            apparition_chance: 100,
            answers: vec![Answer {
                id: 1,
                message: "discussion8_answer1_msg".to_string(),
                ..Default::default()
            }],
            day_system: DaySystem {
                cooldown: 0,
                min_day: 1,
                max_day: Some(1),
            },
        },
        9 => DailyDiscussion {
            id: 9,
            title: "discussion9_title".to_string(),
            description: "discussion9_desc".to_string(),
            image_atlas_index: 9,
            apparition_chance: 100,
            answers: vec![Answer {
                id: 1,
                message: "discussion9_answer1_msg".to_string(),
                equipment_impact: Some(vec![
                    ImpactAction::Add(ItemEnum::Armor(ArmorsEnum::TravelToga.get_armor())),
                    ImpactAction::Add(ItemEnum::Weapon(WeaponsEnum::WoodenSword.get_weapon())),
                ]),
                ..Default::default()
            }],
            day_system: DaySystem {
                cooldown: 0,
                min_day: 1,
                max_day: Some(1),
            },
        },
        _ => panic!(
            "Daily discussion index not found: {}",
            daily_discussion_index
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::*;
    use std::panic;

    #[test]
    fn test_get_daily_discussion_should_panic() {
        let result = panic::catch_unwind(|| {
            get_daily_discussion(&(MAX_DAILY_DISCUSSION_NUMBER + 1));
        });

        assert!(
            result.is_err(),
            "{}",
            "When you update the content, you have to update also the MAX_DAILY_DISCUSSION_NUMBER constant".red()
        );
    }
}
