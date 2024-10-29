use crate::structs::daily_events_folder::{
    daily_events::DaySystem,
    discussions::{Answer, DailyDiscussion},
};

// --- To update whenever the content is updated --- //
const MAX_DAILY_DISCUSSION_NUMBER: u16 = 4;

pub fn get_all_daily_discussions() -> Vec<DailyDiscussion> {
    (1..=MAX_DAILY_DISCUSSION_NUMBER)
        .map(|i| get_daily_discussion(&i))
        .collect()
}

pub fn get_daily_discussion(daily_discussion_index: &u16) -> DailyDiscussion {
    match daily_discussion_index {
        1 => DailyDiscussion {
            id: 1,
            title: "Contempt for the little people".to_string(),
            description: "Ever since your guild arrived, I've had the impression that we little people don't care any more. In your quest for power, have you forgotten about us?".to_string(),
            image_atlas_index: 5,
            apparition_chance: 25,
            answers: vec![
                Answer {
                    experience_impact: None,
                    gold_impact: None,
                    id: 1,
                    message: "We've never forgotten the village. Come to us if you need help.".to_string(),
                    reputation_impact: Some(1),
                    toxicity_impact: Some(-1),
                },
                Answer {
                    experience_impact: None,
                    gold_impact: None,
                    id: 2,
                    message: "It's natural for the strong to progress and the weak to fade away.".to_string(),
                    reputation_impact: Some(-2),
                    toxicity_impact: Some(3),
                },
                Answer {
                    experience_impact: None,
                    gold_impact: None,
                    id: 3,
                    message: "Our recruits are available to anyone who can afford our services. Hehe".to_string(),
                    reputation_impact: Some(-1),
                    toxicity_impact: Some(1),
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
            title: "Shop robbed".to_string(),
            description: "Some rascals tore up my stall chasing a thief. I don't know how I'm going to be able to meet my customer demands. Could you help me solve my problem?".to_string(),
            image_atlas_index: 6,
            apparition_chance: 35,
            answers: vec![
                Answer {
                    id: 1,
                    message: "We'll help you with compensation, we'll make the city safer, we'll make the city safer for everyone.".to_string(),
                    gold_impact: Some(-10),
                    experience_impact: None,
                    toxicity_impact: Some(-3),
                    reputation_impact: Some(3),
                },
                Answer {
                    id: 2,
                    message: "We can't do anything for the moment, but we'll make sure the city is safer.".to_string(),
                    gold_impact: None,
                    experience_impact: None,
                    toxicity_impact: None,
                    reputation_impact: None,
                },
                Answer {
                    id: 3,
                    message: "Wow, it's a good thing it wasn't our place, there'll always be less competition in business, go on, leave me to my business, please!".to_string(),
                    gold_impact: None,
                    experience_impact: None,
                    toxicity_impact: Some(-1),
                    reputation_impact: Some(-2),
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
            title: "Noise pollution".to_string(),
            description: "Hello, I'm part of the neighborhood, your recruits make too much noise at night with their training. Can you ask them to quiet down?".to_string(),
            image_atlas_index: 7,
            apparition_chance: 15,
            answers: vec![
                Answer {
                    id: 1,
                    message: "We'll keep the noise down, and apologize for any inconvenience.".to_string(),
                    gold_impact: None,
                    experience_impact: None,
                    toxicity_impact: None,
                    reputation_impact: Some(1),
                },
                Answer {
                    id: 2,
                    message: "They need training. You'll have to put up with that.".to_string(),
                    gold_impact: Some(0),
                    experience_impact: Some(0),
                    toxicity_impact: Some(0),
                    reputation_impact: Some(-1),
                },
                Answer {
                    id: 3,
                    message: "Hey.. psst come in, let's just say that if I gave you a few golds, we'd forget all about this nasty story?".to_string(),
                    gold_impact: Some(-5),
                    experience_impact: None,
                    toxicity_impact: None,
                    reputation_impact: Some(1),
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
            title: "Fresco by a local artist".to_string(),
            description: "I'd like to immortalize your adventurers in a fresco to decorate the village square.".to_string(),
            image_atlas_index: 8,
            apparition_chance: 5,
            answers: vec![
                Answer {
                    id: 1,
                    message: "What an excellent idea! We will finance this project.".to_string(),
                    gold_impact: Some(-5),
                    experience_impact: None,
                    toxicity_impact: None,
                    reputation_impact: Some(2),
                },
                Answer {
                    id: 2,
                    message: "Sounds good, but you'll have to raise the funds yourself.".to_string(),
                    gold_impact: None,
                    experience_impact: None,
                    toxicity_impact: None,
                    reputation_impact: Some(-1),
                },
                Answer {
                    id: 3,
                    message: "Our employees d.. hum .. I mean our heroes don't need frescoes to prove their worth.".to_string(),
                    gold_impact: None,
                    experience_impact: None,
                    toxicity_impact: Some(1),
                    reputation_impact: Some(-2),
                },
            ],
            day_system: DaySystem {
                cooldown: 15,
                max_day: None,
                min_day: 5,
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
