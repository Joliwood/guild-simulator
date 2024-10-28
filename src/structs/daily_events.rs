#![allow(dead_code)]
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
        let grandma_1 = DailyDiscussionEnum::RandomGrandma1.get_daily_discussion();
        let grandma_2 = DailyDiscussionEnum::RandomGrandma2.get_daily_discussion();
        let grandma_3 = DailyDiscussionEnum::RandomGrandma3.get_daily_discussion();
        let noob_1 = SpontaneousApplicationEnum::RandomNoob1.get_spontaneous_application();
        let noob_2 = SpontaneousApplicationEnum::RandomNoob2.get_spontaneous_application();
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
    percent_chance: u16,
    index: u16,
    day_system: DaySystem,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct SpontaneousApplicationTarget {
    percent_chance: u16,
    index: u16,
    day_system: DaySystem,
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

#[derive(Debug, Component, Resource, Clone, PartialEq, Copy)]
pub enum DailyDiscussionEnum {
    RandomGrandma1,
    RandomGrandma2,
    RandomGrandma3,
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
pub struct SpontaneousApplication {
    pub apparition_chance: u16,
    pub day_system: DaySystem,
    pub description: String,
    pub id: u16,
    pub title: String,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub enum SpontaneousApplicationEnum {
    RandomNoob1,
    RandomNoob2,
}

impl SpontaneousApplicationEnum {
    pub fn get_random_spontaneous_application_enums(
        n: usize,
        player_day: u16,
        daily_event_targets: &mut ResMut<DailyEventTargets>,
    ) -> Vec<SpontaneousApplicationEnum> {
        let available_spontaneous_applications = daily_event_targets
            .daily_spontaneous_application_targets
            .clone();
        let mut selected_spontaneous_applications = Vec::new();

        for _ in 0..n {
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

            if available_spontaneous_applications.is_empty() {
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

            let random_spontaneous_application_enum =
                select_random_spontaneous_application(selected_spontaneous_application.index);
            selected_spontaneous_applications.push(random_spontaneous_application_enum);

            available_spontaneous_applications.remove(selected_index);
        }

        selected_spontaneous_applications
    }

    pub fn get_spontaneous_application(&self) -> SpontaneousApplication {
        match self {
            SpontaneousApplicationEnum::RandomNoob1 => SpontaneousApplication {
                apparition_chance: 75,
                description: "A noob wants to join your guild.".to_string(),
                id: 1,
                title: "Noob 1".to_string(),
                day_system: DaySystem {
                    cooldown: 3,
                    max_day: Some(10),
                    min_day: 1,
                },
            },
            SpontaneousApplicationEnum::RandomNoob2 => SpontaneousApplication {
                apparition_chance: 75,
                description: "A noob wants to join your guild.".to_string(),
                id: 2,
                title: "Noob 2".to_string(),
                day_system: DaySystem {
                    cooldown: 3,
                    max_day: Some(10),
                    min_day: 1,
                },
            },
        }
    }
}

fn select_random_spontaneous_application(index: u16) -> SpontaneousApplicationEnum {
    match index {
        1 => SpontaneousApplicationEnum::RandomNoob1,
        2 => SpontaneousApplicationEnum::RandomNoob2,
        // Should never happen
        _ => SpontaneousApplicationEnum::RandomNoob1,
    }
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

// WIP - OK CHECKED
impl DailyDiscussionEnum {
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

            let random_discussion_enum = select_random_discussion(selected_index as u16);
            selected_discussions.push(random_discussion_enum);

            available_discussions.remove(selected_index);
        }

        selected_discussions
    }

    pub fn get_daily_discussion(&self) -> DailyDiscussion {
        match self {
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
            let daily_discussion = random_discussion_enum.get_daily_discussion();
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
                daily_spontaneous_application_enum.get_spontaneous_application();
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
