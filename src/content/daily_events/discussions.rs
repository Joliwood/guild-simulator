use crate::structs::daily_events_folder::{
    daily_events::DaySystem,
    discussions::{Answer, DailyDiscussion, DailyDiscussionEnum},
};

pub fn get_daily_discussion(daily_discussion_enum: &DailyDiscussionEnum) -> DailyDiscussion {
    match daily_discussion_enum {
        DailyDiscussionEnum::RandomGrandma1 => DailyDiscussion {
            id: 1,
            title: "Curious Grandma".to_string(),
            description: "An old lady approaches with a question about your guild.".to_string(),
            image_atlas_index: 5,
            apparition_chance: 99,
            answers: vec![
                Answer {
                    id: 1,
                    message: "Answer politely.".to_string(),
                    gold_impact: Some(10),
                    experience_impact: Some(5),
                    toxicity_impact: Some(-1),
                },
                Answer {
                    id: 2,
                    message: "Ignore her.".to_string(),
                    gold_impact: Some(-5),
                    experience_impact: Some(0),
                    toxicity_impact: Some(2),
                },
            ],
            day_system: DaySystem {
                cooldown: 7,
                max_day: None,
                min_day: 1,
            },
        },
        DailyDiscussionEnum::RandomGrandma2 => DailyDiscussion {
            id: 2,
            title: "Persistent Grandma".to_string(),
            description: "The same old lady insists on talking to you.".to_string(),
            image_atlas_index: 6,
            apparition_chance: 25,
            answers: vec![
                Answer {
                    id: 3,
                    message: "Offer her some tea.".to_string(),
                    gold_impact: Some(5),
                    experience_impact: Some(10),
                    toxicity_impact: Some(-2),
                },
                Answer {
                    id: 4,
                    message: "Dismiss her.".to_string(),
                    gold_impact: Some(-10),
                    experience_impact: Some(0),
                    toxicity_impact: Some(3),
                },
            ],
            day_system: DaySystem {
                cooldown: 7,
                max_day: None,
                min_day: 1,
            },
        },
        DailyDiscussionEnum::RandomGrandma3 => DailyDiscussion {
            id: 3,
            title: "Suspicious Grandma".to_string(),
            description: "The old lady seems to be hiding something.".to_string(),
            image_atlas_index: 7,
            apparition_chance: 99,
            answers: vec![
                Answer {
                    id: 5,
                    message: "Ask her what she's hiding.".to_string(),
                    gold_impact: Some(0),
                    experience_impact: Some(5),
                    toxicity_impact: Some(1),
                },
                Answer {
                    id: 6,
                    message: "Leave her alone.".to_string(),
                    gold_impact: Some(0),
                    experience_impact: Some(0),
                    toxicity_impact: Some(0),
                },
            ],
            day_system: DaySystem {
                cooldown: 7,
                max_day: None,
                min_day: 3,
            },
        },
    }
}
