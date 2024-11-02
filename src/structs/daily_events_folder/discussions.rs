#![allow(dead_code)]
use super::daily_events::{
    get_random_index_from_percent_arr, DailyEventTargets, DaySystem, DiscussionTarget,
};
use crate::structs::{equipments::ItemEnum, recruits::RecruitStats};
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ImpactAction<T> {
    Add(T),
    Remove(T),
}

#[derive(Default, Debug, Component, Resource, Clone, PartialEq)]
pub struct Answer {
    pub equipment_impact: Option<Vec<ImpactAction<ItemEnum>>>,
    pub experience_impact: Option<u32>,
    pub gold_impact: Option<i32>,
    pub id: u16,
    pub message: String,
    pub recruit_impact: Option<RecruitStats>,
    pub reputation_impact: Option<i8>,
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

pub fn get_random_discussion_indexs(
    n: usize,
    player_day: u16,
    daily_event_targets: &mut ResMut<DailyEventTargets>,
) -> Vec<u16> {
    // We get to compare all the discussions it exists
    let mut available_discussions: Vec<DiscussionTarget> =
        daily_event_targets.daily_discussion_targets.clone();
    let mut selected_discussions = Vec::new();

    for n_index in 0..n {
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

        // If there is no more available spontaneous applications (with min / max day with cooldowns)
        // Or if the random has selected a number of event supérior to the available ones
        if available_discussions.is_empty() || n_index + 1 > available_discussions.len() {
            break;
        }

        let percent_chance_vec = available_discussions
            .iter()
            .map(|discussion| discussion.percent_chance)
            .collect::<Vec<u16>>();

        let selected_index = get_random_index_from_percent_arr(&percent_chance_vec);
        let selected_discussion = available_discussions[selected_index].clone();
        daily_event_targets.update_min_day_for_discussion_by_index(selected_discussion.index);

        selected_discussions.push(selected_discussion.index);

        available_discussions.remove(selected_index);
    }

    info!("Selected discussions : {:?}", selected_discussions);

    selected_discussions
}
