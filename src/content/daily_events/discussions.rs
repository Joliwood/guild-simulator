use crate::{
    content::equipments::{armors::ArmorsEnum, weapons::WeaponsEnum},
    structs::{
        daily_events_folder::{
            daily_events::DaySystem,
            discussions::{Answer, DailyDiscussion, ImpactAction},
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
            title: "Contempt for the little people".to_string(),
            description: "Ever since your guild arrived, I've had the impression that we little people don't care any more. In your quest for power, have you forgotten about us?".to_string(),
            image_atlas_index: 5,
            apparition_chance: 25,
            answers: vec![
                Answer {
                    id: 1,
                    message: "We've never forgotten the village. Come to us if you need help.".to_string(),
                    reputation_impact: Some(1),
                    toxicity_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "It's natural for the strong to progress and the weak to fade away.".to_string(),
                    reputation_impact: Some(-2),
                    toxicity_impact: Some(3),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "Our recruits are available to anyone who can afford our services. Hehe".to_string(),
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
            title: "Shop robbed".to_string(),
            description: "Some rascals tore up my stall chasing a thief. I don't know how I'm going to be able to meet my customer demands. Could you help me solve my problem?".to_string(),
            image_atlas_index: 6,
            apparition_chance: 35,
            answers: vec![
                Answer {
                    id: 1,
                    message: "We'll help you with compensation, we'll make the city safer, we'll make the city safer for everyone.".to_string(),
                    gold_impact: Some(-10),
                    toxicity_impact: Some(-3),
                    reputation_impact: Some(3),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "We can't do anything for the moment, but we'll make sure the city is safer.".to_string(),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "Wow, it's a good thing it wasn't our place, there'll always be less competition in business, go on, leave me to my business, please!".to_string(),
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
            title: "Noise pollution".to_string(),
            description: "Hello, I'm part of the neighborhood, your recruits make too much noise at night with their training. Can you ask them to quiet down?".to_string(),
            image_atlas_index: 7,
            apparition_chance: 15,
            answers: vec![
                Answer {
                    id: 1,
                    message: "We'll keep the noise down, and apologize for any inconvenience.".to_string(),
                    reputation_impact: Some(1),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "They need training. You'll have to put up with that.".to_string(),
                    reputation_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "Hey.. psst come in, let's just say that if I gave you a few golds, we'd forget all about this nasty story?".to_string(),
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
            title: "Fresco by a local artist".to_string(),
            description: "I'd like to immortalize your adventurers in a fresco to decorate the village square.".to_string(),
            image_atlas_index: 8,
            apparition_chance: 5,
            answers: vec![
                Answer {
                    id: 1,
                    message: "What an excellent idea! We will finance this project.".to_string(),
                    gold_impact: Some(-5),
                    reputation_impact: Some(2),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "Sounds good, but you'll have to raise the funds yourself.".to_string(),
                    reputation_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "Our employees d.. hum .. I mean our heroes don't need frescoes to prove their worth.".to_string(),
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
            title: "Village medicines need".to_string(),
            description: "We're running out of medicines for the sick and wounded in the village. Can you help us find some, or, failing that, give us a grant? It's for the common good, you know...".to_string(),
            image_atlas_index: 9,
            apparition_chance: 25,
            answers: vec![
                Answer {
                    id: 1,
                    message: "We'll give you a grant to buy the necessary medicines.".to_string(),
                    gold_impact: Some(-10),
                    toxicity_impact: Some(-2),
                    reputation_impact: Some(2),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "We'll help you find the necessary medicines whenever we'll have the time, mark my words sir !".to_string(),
                    reputation_impact: Some(1),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "We can't help you, but we'll make sure the village is safe.".to_string(),
                    reputation_impact: Some(-1),
                    ..Default::default()
                },
                Answer {
                    id: 4,
                    message: "Common interest, common interest... You're talking to a private company, my dear sir, do you know that?".to_string(),
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
            title: "The miracle worker".to_string(),
            description: "A miracle worker offers to pray for you and your guild, and of course assures you that the ancient gods will only respond if an offering can be made.".to_string(),
            image_atlas_index: 10,
            apparition_chance: 20,
            answers: vec![
                Answer {
                    id: 1,
                    message: "We'll make an offering to the gods. Take this my friend.".to_string(),
                    gold_impact: Some(-5),
                    ..Default::default()
                },
                Answer {
                    id: 2,
                    message: "We don't need the help of the gods, we're doing very well on our own.".to_string(),
                    ..Default::default()
                },
                Answer {
                    id: 3,
                    message: "We'll make an offering to the gods, but we'll make sure it's a good investment.".to_string(),
                    gold_impact: Some(-10),
                    ..Default::default()
                },
                Answer {
                    id: 4,
                    message: "I only believe in finances my dear sir, how do you expect my dear recruits to support themselves if I start doing anything at all ?".to_string(),
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
            title: "Welcome from a grandmother".to_string(),
            description: "A grandmother who tells us that what we're doing is a good thing, that it can bring a little security to the town".to_string(),
            image_atlas_index: 1,
            apparition_chance: 100,
            answers: vec![
                Answer {
                    gold_impact: Some(1),
                    id: 1,
                    message: "Thank you mdam, it's a pleasure".to_string(),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 0,
                min_day: 1,
                max_day: Some(1),
            },
        },
        8 => DailyDiscussion {
            id: 8,
            title: "Mayor's welcome message".to_string(),
            description: "Hello ! I'm delighted that you've accepted my offer to come and live in our beloved town, so welcome! As you know, there's plenty of work in the area. I've managed to find two people for you to recruit, and you should have received their proposals by now. I've provided you with a map of the surrounding area, where we're having a bit of trouble with vagrants causing trouble in the area, so it would be a good start if you could get rid of them for us...".to_string(),
            image_atlas_index: 2,
            apparition_chance: 100,
            answers: vec![
                Answer {
                    gold_impact: Some(50),
                    id: 1,
                    message: "Thank you for welcoming me to your city. I intend to develop this guild with a master's hand, and I'm sure our business will bear fruit.".to_string(),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 0,
                min_day: 1,
                max_day: Some(1),
            },
        },
        9 => DailyDiscussion {
            id: 9,
            title: "Welcome from the merchant guild".to_string(),
            description: "We're delighted to welcome you to the region, and hope your business flourishes. I've registered your guild in our network's merchant directory. That way, they can come and offer you a wide variety of transactions. We hope to hear from you soon.".to_string(),
            image_atlas_index: 3,
            apparition_chance: 100,
            answers: vec![
                Answer {
                    id: 1,
                    message: "Tell him it will be a pleasure to do business with his guild's merchants.".to_string(),
                    ..Default::default()
                },
            ],
            day_system: DaySystem {
                cooldown: 0,
                min_day: 1,
                max_day: Some(1),
            },
        },
        10 => DailyDiscussion {
            id: 10,
            title: "Welcome from a retired warrior".to_string(),
            description: "Hello, I've just seen that you've moved to our beautiful town, so welcome! I've already done my share of work, you know, but perhaps my old equipment could be of use to you, here...".to_string(),
            image_atlas_index: 4,
            apparition_chance: 100,
            answers: vec![
                Answer {
                    id: 1,
                    message: "Thank you very much, you can be sure that we'll make good use of it, have a nice day and please close the drafty door.".to_string(),
                    equipment_impact: Some(vec![
                        ImpactAction::Add(ItemEnum::Armor(ArmorsEnum::ShieldOfCourage.get_armor())),
                        ImpactAction::Add(ItemEnum::Weapon(WeaponsEnum::SpearOfDestiny.get_weapon()))
                    ]),
                    ..Default::default()
                },
            ],
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
