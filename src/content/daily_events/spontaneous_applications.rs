use crate::{
    enums::{RecruitEnum, RecruitStateEnum},
    structs::{
        daily_events_folder::{
            daily_events::DaySystem, discussions::Answer,
            spontaneous_applications::SpontaneousApplication,
        },
        recruits::{RecruitInventory, RecruitStats},
    },
};
use uuid::Uuid;

// --- To update whenever the content is updated --- //
const MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER: u16 = 3;

pub fn get_all_spontaneous_applications() -> Vec<SpontaneousApplication> {
    (1..=MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER)
        .map(|i| get_spontaneous_application(&i))
        .collect()
}

pub fn get_spontaneous_application(spontaneous_application_index: &u16) -> SpontaneousApplication {
    match spontaneous_application_index {
        1 => SpontaneousApplication {
            apparition_chance: 100,
            description: "Hello, the mayor sent me because I have some humm... worries let's say... In short, I've got debts to pay, mainly due to the increase in beer prices at the inn, which I didn't insure... I'm a new man, believe me! Do you have a job for a repentant man?".to_string(),
            id: 1,
            image_atlas_index: 0,
            title: "Proposal from a person in debt".to_string(),
            day_system: DaySystem {
                cooldown: 0,
                max_day: Some(1),
                min_day: 1,
            },
            answers: vec![
                Answer {
                    id: 1,
                    message: "Welcome to our guild ! You're hired !".to_string(),
                    recruit_impact: Some(RecruitStats {
                        class: RecruitEnum::Warrior,
                        endurance: 4,
                        experience: 0,
                        id: Uuid::new_v4(),
                        image_atlas_index: 0,
                        intelligence: 2,
                        level: 1,
                        max_experience: 100,
                        name: "Jean-Louis-David".to_string(),
                        recruit_inventory: RecruitInventory {
                            armor: None,
                            weapon: None,
                            scrolls: vec![],
                        },
                        state: RecruitStateEnum::Available,
                        strength: 5,
                    }),
                    ..Default::default()
                },
            ],
        },
        2 => SpontaneousApplication {
            apparition_chance: 100,
            description: "Hello ! So you're the new club on the block? Well, I was sent here because I made some blunders... So here's my confession: I set fire to the library, but I didn't mean to! You've got to believe me... at the same time, if the town had installed street lighting, I wouldn't have been clutching an oil lamp as I slid down the library steps... I think I'll have to work for a while to pay off my debt to the city...".to_string(),
            id: 2,
            image_atlas_index: 1,
            title: "He says his name is Hubert...".to_string(),
            day_system: DaySystem {
                cooldown: 0,
                max_day: Some(1),
                min_day: 1,
            },
            answers: vec![
                Answer {
                    id: 1,
                    message: "I see you are a mage but with strength and endurance stats, what is that seriously ? Well I need recruits, come in.. I mean welcome".to_string(),
                    recruit_impact: Some(RecruitStats {
                        class: RecruitEnum::Mage,
                        endurance: 8,
                        experience: 0,
                        id: Uuid::new_v4(),
                        image_atlas_index: 1,
                        intelligence: 0,
                        level: 1,
                        max_experience: 100,
                        name: "Hubert".to_string(),
                        recruit_inventory: RecruitInventory {
                            armor: None,
                            weapon: None,
                            scrolls: vec![],
                        },
                        state: RecruitStateEnum::Available,
                        strength: 12,
                    }),
                    ..Default::default()
                },
            ],
        },
        3 => SpontaneousApplication {
            apparition_chance: 70,
            description: "Hello, I am not really good, but I can do things... I suppose... but nothing is free !".to_string(),
            id: 3,
            image_atlas_index: 0,
            title: "A big noob approach".to_string(),
            day_system: DaySystem {
                cooldown: 3,
                max_day: None,
                min_day: 2,
            },
            answers: vec![
                Answer {
                    id: 1,
                    message: "We have some dirty missions, let's go !".to_string(),
                    gold_impact: Some(-20),
                    recruit_impact: Some(RecruitStats {
                        class: RecruitEnum::Rogue,
                        endurance: 5,
                        experience: 0,
                        id: Uuid::new_v4(),
                        image_atlas_index: 3,
                        intelligence: 1,
                        level: 1,
                        max_experience: 100,
                        name: "Big noob".to_string(),
                        recruit_inventory: RecruitInventory {
                            armor: None,
                            weapon: None,
                            scrolls: vec![],
                        },
                        state: RecruitStateEnum::Available,
                        strength: 5,
                    }),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "I don't have time for you sorry, have a good day".to_string(),
                    ..Default::default()
                },
            ],
        },
        _ => panic!(
            "Spontaneous application index not found: {}",
            spontaneous_application_index
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::*;
    use std::panic;

    #[test]
    fn test_get_spontaneous_application_should_panic() {
        let result = panic::catch_unwind(|| {
            get_spontaneous_application(&(MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER + 1));
        });

        assert!(
            result.is_err(),
            "{}",
            "When you update the content, you have to update also the MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER constant".red()
        );
    }
}
