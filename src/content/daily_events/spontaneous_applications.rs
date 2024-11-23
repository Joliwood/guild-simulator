use crate::{
    content::recruits::RecruitEnum,
    structs::daily_events_folder::{
        daily_events::{Answer, DaySystem},
        spontaneous_applications::SpontaneousApplication,
    },
};

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
            description: "spontaneous_app1_desc".to_string(),
            id: 1,
            image_atlas_index: 0,
            title: "spontaneous_app1_title".to_string(),
            day_system: DaySystem {
                cooldown: 0,
                max_day: Some(1),
                min_day: 1,
            },
            answers: vec![Answer {
                id: 1,
                message: "spontaneous_app2_answer1_msg".to_string(),
                recruit_impact: Some(RecruitEnum::JeanLouisDavid.get_recruit()),
                ..Default::default()
            }],
        },
        2 => SpontaneousApplication {
            apparition_chance: 100,
            description: "spontaneous_app2_desc".to_string(),
            id: 2,
            image_atlas_index: 1,
            title: "spontaneous_app2_title".to_string(),
            day_system: DaySystem {
                cooldown: 0,
                max_day: Some(1),
                min_day: 1,
            },
            answers: vec![Answer {
                id: 1,
                message: "spontaneous_app2_answer1_msg".to_string(),
                recruit_impact: Some(RecruitEnum::Hubert.get_recruit()),
                ..Default::default()
            }],
        },
        3 => SpontaneousApplication {
            apparition_chance: 70,
            description: "spontaneous_app3_desc".to_string(),
            id: 3,
            image_atlas_index: 0,
            title: "spontaneous_app3_title".to_string(),
            day_system: DaySystem {
                cooldown: 3,
                max_day: None,
                min_day: 2,
            },
            answers: vec![
                Answer {
                    id: 1,
                    message: "spontaneous_app3_answer1_msg".to_string(),
                    gold_impact: Some(-20),
                    recruit_impact: Some(RecruitEnum::BigNoob.get_recruit()),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "spontaneous_app3_answer2_msg".to_string(),
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
