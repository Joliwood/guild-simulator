use super::daily_events::{
    get_random_index_from_percent_arr, DailyEventTargets, DaySystem, DiscussionTarget,
};
use bevy::prelude::*;

#[derive(Debug, Component, Resource, Clone, PartialEq, Copy)]
pub enum DailyDiscussionEnum {
    RandomGrandma1,
    RandomGrandma2,
    RandomGrandma3,
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
    pub description: String,
    pub id: u16,
    pub image_atlas_index: u16,
    pub title: String,
    pub day_system: DaySystem,
}

impl DailyDiscussionEnum {
    pub fn select_random_discussion(index: u16) -> DailyDiscussionEnum {
        match index {
            1 => DailyDiscussionEnum::RandomGrandma1,
            2 => DailyDiscussionEnum::RandomGrandma2,
            3 => DailyDiscussionEnum::RandomGrandma3,
            // Should never happen
            _ => DailyDiscussionEnum::RandomGrandma1,
        }
    }

    pub fn get_random_discussion_enums(
        n: usize,
        player_day: u16,
        daily_event_targets: &mut ResMut<DailyEventTargets>,
    ) -> Vec<DailyDiscussionEnum> {
        // We get to compare all the discussions it exists
        let mut available_discussions: Vec<DiscussionTarget> =
            daily_event_targets.daily_discussion_targets.clone();
        let mut selected_discussions = Vec::new();

        for _ in 0..n {
            // We update the available discussions with only the ones that fit the player day
            available_discussions = available_discussions
                .iter()
                .filter(|discussion| {
                    player_day >= discussion.day_system.min_day
                        && discussion
                            .day_system
                            .max_day
                            .map_or(true, |max_day| player_day <= max_day)
                })
                .cloned()
                .collect::<Vec<_>>();

            if available_discussions.is_empty() {
                break;
            }

            let percent_chance_vec = available_discussions
                .iter()
                .map(|discussion| discussion.percent_chance)
                .collect::<Vec<u16>>();

            let selected_index = get_random_index_from_percent_arr(&percent_chance_vec);
            let selected_discussion = available_discussions[selected_index].clone();
            daily_event_targets.update_min_day_for_discussion_by_index(selected_discussion.index);

            let random_discussion_enum = Self::select_random_discussion(selected_index as u16);
            selected_discussions.push(random_discussion_enum);

            available_discussions.remove(selected_index);
        }

        selected_discussions
    }
}
