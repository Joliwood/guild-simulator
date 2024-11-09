use super::daily_events::{
    get_random_index_from_percent_arr, Answer, DailyEventTargets, DaySystem,
};
use bevy::prelude::*;

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct SpontaneousApplication {
    pub answers: Vec<Answer>,
    pub apparition_chance: u16,
    pub day_system: DaySystem,
    pub description: String,
    pub id: u16,
    pub image_atlas_index: u16,
    pub title: String,
}

pub fn get_random_spontaneous_application_indexs(
    n: usize,
    player_day: u16,
    daily_event_targets: &mut ResMut<DailyEventTargets>,
) -> Vec<u16> {
    let available_spontaneous_applications = daily_event_targets
        .daily_spontaneous_application_targets
        .clone();
    let mut selected_spontaneous_applications = Vec::new();

    for n_index in 0..n {
        let mut available_spontaneous_applications = available_spontaneous_applications
            .iter()
            .filter(|application| {
                player_day >= application.day_system.min_day
                    && application
                        .day_system
                        .max_day
                        .map_or(true, |max_day| player_day <= max_day)
            })
            .cloned()
            .collect::<Vec<_>>();

        // If there is no more available spontaneous applications (with min / max day with cooldowns)
        // Or if the random has selected a number of event supérior to the available ones
        if available_spontaneous_applications.is_empty()
            || n_index + 1 > available_spontaneous_applications.len()
        {
            break;
        }

        let percent_chance_vec = available_spontaneous_applications
            .iter()
            .map(|application| application.percent_chance)
            .collect::<Vec<u16>>();

        let selected_index = get_random_index_from_percent_arr(&percent_chance_vec);
        let selected_spontaneous_application =
            available_spontaneous_applications[selected_index].clone();

        daily_event_targets.update_min_day_for_spontaneous_application_by_index(
            selected_spontaneous_application.index,
        );

        selected_spontaneous_applications.push(selected_spontaneous_application.index);

        available_spontaneous_applications.remove(selected_index);
    }

    return selected_spontaneous_applications;
}
