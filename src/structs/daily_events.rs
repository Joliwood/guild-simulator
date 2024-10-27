#![allow(dead_code)]
use bevy::prelude::*;
use rand::Rng;

pub static TOTAL_APPARITION_CHANCE: u16 = calculate_total_apparition_chance();

const fn calculate_total_apparition_chance() -> u16 {
    let mut total = 0;
    let mut i = 0;
    while i < ALL_LISTED_APPARITION_CHANCE.len() {
        total += ALL_LISTED_APPARITION_CHANCE[i];
        i += 1;
    }
    total
}

const ALL_LISTED_APPARITION_CHANCE: [u16; 3] = [99, 5, 99];

pub fn get_random_index() -> usize {
    let random_number = rand::thread_rng().gen_range(0..TOTAL_APPARITION_CHANCE);
    let mut cumulative = 0;

    for (index, &chance) in ALL_LISTED_APPARITION_CHANCE.iter().enumerate() {
        cumulative += chance;
        if random_number < cumulative {
            return index;
        }
    }
    // Default to the last element if none were selected (shouldn't happen)
    ALL_LISTED_APPARITION_CHANCE.len() - 1
}

pub struct Answer {
    pub id: u16,
    pub message: String,
    pub gold_impact: Option<i32>,
    pub experience_impact: Option<u32>,
    pub toxicity_impact: Option<i8>,
}

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

impl DailyDiscussionEnum {
    pub fn get_random_discussion() -> DailyDiscussionEnum {
        match get_random_index() {
            0 => DailyDiscussionEnum::RandomGrandma1,
            1 => DailyDiscussionEnum::RandomGrandma2,
            2 => DailyDiscussionEnum::RandomGrandma3,
            // Should never happen
            _ => DailyDiscussionEnum::RandomGrandma1,
        }
    }

    pub fn get_daily_iscussion(&self) -> DailyDiscussion {
        match self {
            DailyDiscussionEnum::RandomGrandma1 => DailyDiscussion {
                id: 1,
                title: "Curious Grandma".to_string(),
                description: "An old lady approaches with a question about your guild.".to_string(),
                image_atlas_index: 5,
                apparition_chance: ALL_LISTED_APPARITION_CHANCE[0],
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
                id: 2,
                title: "Persistent Grandma".to_string(),
                description: "The same old lady insists on talking to you.".to_string(),
                image_atlas_index: 6,
                apparition_chance: ALL_LISTED_APPARITION_CHANCE[1],
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
                id: 3,
                title: "Suspicious Grandma".to_string(),
                description: "The old lady seems to be hiding something.".to_string(),
                image_atlas_index: 7,
                apparition_chance: ALL_LISTED_APPARITION_CHANCE[2],
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
                discussion_enum.get_daily_iscussion().apparition_chance
            }
        }
    }
}

#[derive(Default, Debug, Component, Resource, Clone)]
pub struct DailyEvents(pub Vec<DailyEvent>);

impl DailyEvents {}
