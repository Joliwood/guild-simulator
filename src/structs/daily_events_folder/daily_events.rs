use super::{
    discussions::DailyDiscussionEnum, spontaneous_applications::SpontaneousApplicationEnum,
};
use crate::content::daily_events::{
    discussions::get_daily_discussion, spontaneous_applications::get_spontaneous_application,
};
use bevy::prelude::*;
use rand::Rng;

fn calculate_total_apparition_chance(list: &[u16]) -> u16 {
    let mut total = 0;
    let mut i = 0;
    while i < list.len() {
        total += list[i];
        i += 1;
    }
    total
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DailyEventTargets {
    pub daily_discussion_targets: Vec<DiscussionTarget>,
    pub daily_spontaneous_application_targets: Vec<SpontaneousApplicationTarget>,
}

impl Default for DailyEventTargets {
    fn default() -> Self {
        let grandma_1 = get_daily_discussion(&DailyDiscussionEnum::RandomGrandma1);
        let grandma_2 = get_daily_discussion(&DailyDiscussionEnum::RandomGrandma2);
        let grandma_3 = get_daily_discussion(&DailyDiscussionEnum::RandomGrandma3);
        let noob_1 = get_spontaneous_application(&SpontaneousApplicationEnum::RandomNoob1);
        let noob_2 = get_spontaneous_application(&SpontaneousApplicationEnum::RandomNoob2);
        Self {
            daily_discussion_targets: vec![
                // Grandma 1
                DiscussionTarget {
                    percent_chance: grandma_1.apparition_chance,
                    index: grandma_1.id,
                    day_system: grandma_1.day_system,
                },
                // Grandma 2
                DiscussionTarget {
                    percent_chance: grandma_2.apparition_chance,
                    index: grandma_2.id,
                    day_system: grandma_2.day_system,
                },
                // Grandma 3
                DiscussionTarget {
                    percent_chance: grandma_3.apparition_chance,
                    index: grandma_3.id,
                    day_system: grandma_3.day_system,
                },
            ],
            daily_spontaneous_application_targets: vec![
                // Noob 1
                SpontaneousApplicationTarget {
                    percent_chance: noob_1.apparition_chance,
                    index: noob_1.id,
                    day_system: noob_1.day_system,
                },
                // Noob 2
                SpontaneousApplicationTarget {
                    percent_chance: noob_2.apparition_chance,
                    index: noob_2.id,
                    day_system: noob_2.day_system,
                },
            ],
        }
    }
}

impl DailyEventTargets {
    pub fn update_min_day_for_spontaneous_application_by_index(&mut self, index: u16) {
        let target = self
            .daily_spontaneous_application_targets
            .iter_mut()
            .find(|target| target.index == index)
            .unwrap();

        target.day_system.update_min_day();
    }

    pub fn update_min_day_for_discussion_by_index(&mut self, index: u16) {
        let target = self
            .daily_discussion_targets
            .iter_mut()
            .find(|target| target.index == index)
            .unwrap();

        target.day_system.update_min_day();
    }
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DiscussionTarget {
    pub percent_chance: u16,
    pub index: u16,
    pub day_system: DaySystem,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct SpontaneousApplicationTarget {
    pub percent_chance: u16,
    pub index: u16,
    pub day_system: DaySystem,
}

pub fn get_random_index_from_percent_arr(list: &[u16]) -> usize {
    let total_apparition_chance = calculate_total_apparition_chance(list);

    let random_number = rand::thread_rng().gen_range(0..total_apparition_chance);
    let mut cumulative = 0;

    for (index, &chance) in list.iter().enumerate() {
        cumulative += chance;
        if random_number <= cumulative {
            return index;
        }
    }

    // Default to the last element if none were selected (shouldn't happen)
    list.len() - 1
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DaySystem {
    pub cooldown: u16,
    pub max_day: Option<u16>,
    pub min_day: u16,
}

impl DaySystem {
    pub fn update_min_day(&mut self) {
        self.min_day += self.cooldown;
    }
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub enum DailyEventTypeEnum {
    Discussion(DailyDiscussionEnum),
    // EquipmentTrade(u16),
    // MapProposition(u16),
    SpontaneousApplication(SpontaneousApplicationEnum),
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DailyEvent {
    pub daily_event_type: DailyEventTypeEnum,
    pub day_system: DaySystem,
}

#[derive(Default, Debug, Component, Resource, Clone)]
pub struct DailyEvents(pub Vec<DailyEvent>);

impl DailyEvents {
    pub fn get_last_daily_event(&self) -> Option<&DailyEvent> {
        self.0.last()
    }

    pub fn get_random_number_of_daily_events(
        &self,
        n: usize,
        player_day: u16,
        daily_event_targets: &mut ResMut<DailyEventTargets>,
    ) -> Vec<DailyEvent> {
        let mut daily_events: Vec<DailyEvent> = Vec::new();
        let mut daily_discussion_number = 0;
        let mut daily_spontaneous_application_number = 0;

        // Discussion has a 66% chance of being selected.
        // Spontaneous application has a 33% chance of being selected.

        for _ in 0..n {
            let random_number = rand::thread_rng().gen_range(0..100);
            if random_number <= 66 {
                daily_discussion_number += 1;
            } else {
                daily_spontaneous_application_number += 1;
            }
        }

        let random_discussion_enums = DailyDiscussionEnum::get_random_discussion_enums(
            daily_discussion_number,
            player_day,
            daily_event_targets,
        );

        for random_discussion_enum in random_discussion_enums {
            let daily_discussion = get_daily_discussion(&random_discussion_enum);
            let daily_event = DailyEvent {
                daily_event_type: DailyEventTypeEnum::Discussion(random_discussion_enum),
                day_system: daily_discussion.day_system.clone(),
            };
            daily_events.push(daily_event);
        }

        let daily_sponaneous_applications =
            SpontaneousApplicationEnum::get_random_spontaneous_application_enums(
                daily_spontaneous_application_number,
                player_day,
                daily_event_targets,
            );

        for daily_spontaneous_application_enum in daily_sponaneous_applications {
            let daily_spontaneous_application =
                get_spontaneous_application(&daily_spontaneous_application_enum);
            let daily_event = DailyEvent {
                daily_event_type: DailyEventTypeEnum::SpontaneousApplication(
                    daily_spontaneous_application_enum,
                ),
                day_system: daily_spontaneous_application.day_system.clone(),
            };
            daily_events.push(daily_event);
        }

        daily_events
    }
}
