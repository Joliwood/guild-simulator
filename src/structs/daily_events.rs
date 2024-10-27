#![allow(dead_code)]
use bevy::prelude::*;
use rand::Rng;

// pub static TOTAL_APPARITION_CHANCE: u16 =
//     calculate_total_apparition_chance(&ALL_DISCUSSIONS_LISTED_APPARITION_CHANCE);

fn calculate_total_apparition_chance(list: &[u16]) -> u16 {
    let mut total = 0;
    let mut i = 0;
    while i < list.len() {
        total += list[i];
        i += 1;
    }
    total
}

// const RANDOM_GRANDMA_1_APPARITION_CHANCE: u16 = 99;
// const RANDOM_GRANDMA_2_APPARITION_CHANCE: u16 = 5;
// const RANDOM_GRANDMA_3_APPARITION_CHANCE: u16 = 99;

#[derive(Debug, Component, Resource, Clone, PartialEq)]
struct DiscussionTarget {
    percent_chance: u16,
    index: u16,
}

const RANDOM_GRANDMA_1: DiscussionTarget = DiscussionTarget {
    percent_chance: 99,
    index: 1,
};

const RANDOM_GRANDMA_2: DiscussionTarget = DiscussionTarget {
    percent_chance: 25,
    index: 2,
};

const RANDOM_GRANDMA_3: DiscussionTarget = DiscussionTarget {
    percent_chance: 99,
    index: 3,
};

const GRANDMAS_INITIAL_LIST: [DiscussionTarget; 3] =
    [RANDOM_GRANDMA_1, RANDOM_GRANDMA_2, RANDOM_GRANDMA_3];

const ALL_DISCUSSIONS_LISTED_APPARITION_CHANCE: [u16; 3] = [
    RANDOM_GRANDMA_1.percent_chance,
    RANDOM_GRANDMA_2.percent_chance,
    RANDOM_GRANDMA_3.percent_chance,
];

pub fn get_random_index(list: &[u16]) -> usize {
    let total_apparition_chance = calculate_total_apparition_chance(list);

    let random_number = rand::thread_rng().gen_range(0..total_apparition_chance);
    info!("random_number : {:?}", random_number);
    let mut cumulative = 0;

    for (index, &chance) in list.iter().enumerate() {
        info!("index in the loop : {:?}", index);
        cumulative += chance;
        if random_number <= cumulative {
            return index;
        }
    }
    // Default to the last element if none were selected (shouldn't happen)
    list.len() - 1
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct Answer {
    pub id: u16,
    pub message: String,
    pub gold_impact: Option<i32>,
    pub experience_impact: Option<u32>,
    pub toxicity_impact: Option<i8>,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DailyDiscussion {
    pub answers: Vec<Answer>,
    pub apparition_chance: u16,
    pub cooldown: u16,
    pub description: String,
    pub id: u16,
    pub image_atlas_index: u16,
    pub max_day: u16,
    pub min_day: u16,
    pub title: String,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub enum DailyDiscussionEnum {
    RandomGrandma1,
    RandomGrandma2,
    RandomGrandma3,
}

fn select_random_discussion(index: u16) -> DailyDiscussionEnum {
    match index {
        1 => DailyDiscussionEnum::RandomGrandma1,
        2 => DailyDiscussionEnum::RandomGrandma2,
        3 => DailyDiscussionEnum::RandomGrandma3,
        // Should never happen
        _ => DailyDiscussionEnum::RandomGrandma1,
    }
}

impl DailyDiscussionEnum {
    pub fn get_two_random_discussions() -> (DailyDiscussionEnum, DailyDiscussionEnum) {
        let first_list = GRANDMAS_INITIAL_LIST.to_vec();

        let first_percent_chance_vec = first_list
            .iter()
            .map(|discussion_target| discussion_target.percent_chance)
            .collect::<Vec<u16>>();
        let first_index = get_random_index(&first_percent_chance_vec);
        let first_random_discussion_index = first_list[first_index].index;

        let first_random_discussion = select_random_discussion(first_random_discussion_index);

        let first_random_discussion_id = first_random_discussion.clone().get_daily_discussion().id;

        let second_list = first_list
            .iter()
            .enumerate()
            .filter(|(_, discussion_target)| discussion_target.index != first_random_discussion_id)
            .map(|(_, discussion_target)| discussion_target)
            .collect::<Vec<&DiscussionTarget>>();

        let second_percent_chance_vec = second_list
            .iter()
            .map(|discussion_target| discussion_target.percent_chance)
            .collect::<Vec<u16>>();

        let second_index = get_random_index(&second_percent_chance_vec);
        let second_random_discussion_index = second_list[second_index].index;

        let second_random_discussion = select_random_discussion(second_random_discussion_index);

        return (first_random_discussion, second_random_discussion);
    }

    pub fn get_random_discussions(n: usize) -> Vec<DailyDiscussionEnum> {
        // Convert the initial list to a modifiable vector.
        let mut available_discussions = GRANDMAS_INITIAL_LIST.to_vec();
        let mut selected_discussions = Vec::new();

        for _ in 0..n {
            // Collect the apparition chances for the current set of available discussions.
            let percent_chance_vec = available_discussions
                .iter()
                .map(|discussion_target| discussion_target.percent_chance)
                .collect::<Vec<u16>>();

            // Select a random index based on apparition chance.
            let selected_index = get_random_index(&percent_chance_vec);
            let selected_discussion = available_discussions[selected_index].clone();

            // Convert the selected discussion's index to `DailyDiscussionEnum`.
            let random_discussion_enum = select_random_discussion(selected_discussion.index);
            selected_discussions.push(random_discussion_enum);

            // Remove the selected discussion from the available list for uniqueness.
            available_discussions.remove(selected_index);

            // Stop early if there are no more discussions left to select.
            if available_discussions.is_empty() {
                break;
            }
        }

        selected_discussions
    }

    pub fn get_daily_discussion(&self) -> DailyDiscussion {
        match self {
            DailyDiscussionEnum::RandomGrandma1 => DailyDiscussion {
                id: RANDOM_GRANDMA_1.index,
                title: "Curious Grandma".to_string(),
                description: "An old lady approaches with a question about your guild.".to_string(),
                image_atlas_index: 5,
                apparition_chance: RANDOM_GRANDMA_1.percent_chance,
                cooldown: 3,
                max_day: 10,
                min_day: 1,
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
            },
            DailyDiscussionEnum::RandomGrandma2 => DailyDiscussion {
                id: RANDOM_GRANDMA_2.index,
                title: "Persistent Grandma".to_string(),
                description: "The same old lady insists on talking to you.".to_string(),
                image_atlas_index: 6,
                apparition_chance: RANDOM_GRANDMA_2.percent_chance,
                cooldown: 3,
                max_day: 10,
                min_day: 1,
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
            },
            DailyDiscussionEnum::RandomGrandma3 => DailyDiscussion {
                id: RANDOM_GRANDMA_3.index,
                title: "Suspicious Grandma".to_string(),
                description: "The old lady seems to be hiding something.".to_string(),
                image_atlas_index: 7,
                apparition_chance: RANDOM_GRANDMA_3.percent_chance,
                cooldown: 3,
                max_day: 10,
                min_day: 1,
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
            },
        }
    }
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub enum DailyEventTypeEnum {
    Discussion(DailyDiscussionEnum),
    // EquipmentTrade(u16),
    // MapProposition(u16),
    // SpontaneousApplication(u16),
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DailyEvent {
    pub id: u16,
    pub daily_event_type: DailyEventTypeEnum,
}

impl DailyEvent {
    pub fn get_random_daily_event_enum() -> DailyEventTypeEnum {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..3);
        match random_number {
            0 => DailyEventTypeEnum::Discussion(DailyDiscussionEnum::RandomGrandma1),
            1 => DailyEventTypeEnum::Discussion(DailyDiscussionEnum::RandomGrandma2),
            2 => DailyEventTypeEnum::Discussion(DailyDiscussionEnum::RandomGrandma3),
            _ => DailyEventTypeEnum::Discussion(DailyDiscussionEnum::RandomGrandma1),
        }
    }

    pub fn get_percent_chance(&self) -> u16 {
        match &self.daily_event_type {
            DailyEventTypeEnum::Discussion(discussion_enum) => {
                discussion_enum.get_daily_discussion().apparition_chance
            }
        }
    }
}

#[derive(Default, Debug, Component, Resource, Clone)]
pub struct DailyEvents(pub Vec<DailyEvent>);

impl DailyEvents {
    // pub fn get_random_discussion(&self) -> DailyDiscussion {
    //     let random_discussion_event = DailyDiscussionEnum::get_random_discussion();
    //     random_discussion_event.get_daily_discussion()
    // }
}
