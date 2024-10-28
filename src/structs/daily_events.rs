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
struct DiscussionTarget {
    percent_chance: u16,
    index: u16,
    day_system: DaySystem,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
struct SpontaneousApplicationTarget {
    percent_chance: u16,
    index: u16,
    day_system: DaySystem,
}

const RANDOM_GRANDMA_1: DiscussionTarget = DiscussionTarget {
    percent_chance: 99,
    index: 1,
    day_system: DaySystem {
        cooldown: 3,
        max_day: 10,
        min_day: 1,
    },
};

const RANDOM_GRANDMA_2: DiscussionTarget = DiscussionTarget {
    percent_chance: 25,
    index: 2,
    day_system: DaySystem {
        cooldown: 3,
        max_day: 10,
        min_day: 1,
    },
};

const RANDOM_GRANDMA_3: DiscussionTarget = DiscussionTarget {
    percent_chance: 99,
    index: 3,
    day_system: DaySystem {
        cooldown: 2,
        max_day: 15,
        min_day: 3,
    },
};

const GRANDMAS_INITIAL_LIST: [DiscussionTarget; 3] =
    [RANDOM_GRANDMA_1, RANDOM_GRANDMA_2, RANDOM_GRANDMA_3];

const RANDOM_NOOB_1: SpontaneousApplicationTarget = SpontaneousApplicationTarget {
    percent_chance: 100,
    index: 1,
    day_system: DaySystem {
        cooldown: 3,
        max_day: 10,
        min_day: 1,
    },
};

const RANDOM_NOOB_2: SpontaneousApplicationTarget = SpontaneousApplicationTarget {
    percent_chance: 100,
    index: 2,
    day_system: DaySystem {
        cooldown: 3,
        max_day: 10,
        min_day: 1,
    },
};

const SPONTANOUS_APPLICATIONS_INITIAL_LIST: [SpontaneousApplicationTarget; 2] =
    [RANDOM_NOOB_1, RANDOM_NOOB_2];

pub fn get_random_index(list: &[u16]) -> usize {
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
    pub max_day: u16,
    pub min_day: u16,
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
    // pub fn get_random_spontaneous_application_enums(n: usize) -> Vec<SpontaneousApplicationEnum> {
    //     // Convert the initial list to a modifiable vector.
    //     let mut available_spontaneous_applications = SPONTANOUS_APPLICATIONS_INITIAL_LIST.to_vec();
    //     let mut selected_spontaneous_applications = Vec::new();

    //     for _ in 0..n {
    //         // Collect the apparition chances for the current set of available spontaneous applications.
    //         let percent_chance_vec = available_spontaneous_applications
    //             .iter()
    //             .map(|spontaneous_application_target| spontaneous_application_target.percent_chance)
    //             .collect::<Vec<u16>>();

    //         // Select a random index based on apparition chance.
    //         let selected_index = get_random_index(&percent_chance_vec);
    //         let selected_spontaneous_application =
    //             available_spontaneous_applications[selected_index].clone();

    //         // Convert the selected spontaneous application's index to `SpontaneousApplicationEnum`.
    //         let random_spontaneous_application_enum =
    //             select_random_spontaneous_application(selected_spontaneous_application.index);
    //         selected_spontaneous_applications.push(random_spontaneous_application_enum);

    //         // Remove the selected spontaneous application from the available list for uniqueness.
    //         available_spontaneous_applications.remove(selected_index);

    //         // Stop early if there are no more spontaneous applications left to select.
    //         if available_spontaneous_applications.is_empty() {
    //             break;
    //         }
    //     }

    //     selected_spontaneous_applications
    // }

    pub fn get_random_spontaneous_application_enums(
        n: usize,
        player_day: u16,
    ) -> Vec<SpontaneousApplicationEnum> {
        let mut available_spontaneous_applications = SPONTANOUS_APPLICATIONS_INITIAL_LIST.to_vec();
        let mut selected_spontaneous_applications = Vec::new();

        for _ in 0..n {
            let mut available_spontaneous_applications = available_spontaneous_applications
                .iter()
                .filter(|application| {
                    player_day >= application.day_system.min_day
                        && player_day <= application.day_system.max_day
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

            let selected_index = get_random_index(&percent_chance_vec);
            let selected_spontaneous_application =
                available_spontaneous_applications[selected_index].clone();

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
                apparition_chance: RANDOM_NOOB_1.percent_chance,
                description: "A noob wants to join your guild.".to_string(),
                id: RANDOM_NOOB_1.index,
                title: "Noob 1".to_string(),
                day_system: DaySystem {
                    cooldown: RANDOM_NOOB_1.day_system.cooldown,
                    max_day: RANDOM_NOOB_1.day_system.max_day,
                    min_day: RANDOM_NOOB_1.day_system.min_day,
                },
            },
            SpontaneousApplicationEnum::RandomNoob2 => SpontaneousApplication {
                apparition_chance: RANDOM_NOOB_2.percent_chance,
                description: "A noob wants to join your guild.".to_string(),
                id: RANDOM_NOOB_2.index,
                title: "Noob 2".to_string(),
                day_system: DaySystem {
                    cooldown: RANDOM_NOOB_2.day_system.cooldown,
                    max_day: RANDOM_NOOB_2.day_system.max_day,
                    min_day: RANDOM_NOOB_2.day_system.min_day,
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

impl DailyDiscussionEnum {
    // pub fn get_random_discussion_enums(n: usize) -> Vec<DailyDiscussionEnum> {
    //     // Convert the initial list to a modifiable vector.
    //     let mut available_discussions = GRANDMAS_INITIAL_LIST.to_vec();
    //     let mut selected_discussions = Vec::new();

    //     for _ in 0..n {
    //         // Collect the apparition chances for the current set of available discussions.
    //         let percent_chance_vec = available_discussions
    //             .iter()
    //             .map(|discussion_target| discussion_target.percent_chance)
    //             .collect::<Vec<u16>>();

    //         // Select a random index based on apparition chance.
    //         let selected_index = get_random_index(&percent_chance_vec);
    //         let selected_discussion = available_discussions[selected_index].clone();

    //         // Convert the selected discussion's index to `DailyDiscussionEnum`.
    //         let random_discussion_enum = select_random_discussion(selected_discussion.index);
    //         selected_discussions.push(random_discussion_enum);

    //         // Remove the selected discussion from the available list for uniqueness.
    //         available_discussions.remove(selected_index);

    //         // Stop early if there are no more discussions left to select.
    //         if available_discussions.is_empty() {
    //             break;
    //         }
    //     }

    //     selected_discussions
    // }

    pub fn get_random_discussion_enums(n: usize, player_day: u16) -> Vec<DailyDiscussionEnum> {
        let mut available_discussions = GRANDMAS_INITIAL_LIST.to_vec();
        let mut selected_discussions = Vec::new();

        // let new_available_discussions = available_discussions
        //     .iter()
        //     .filter(|discussion| {
        //         player_day >= discussion.day_system.min_day
        //             && player_day <= discussion.day_system.max_day
        //     })
        //     .cloned()
        //     .collect::<Vec<_>>();

        for _ in 0..n {
            info!(
                "Checking day range with player_day: {}. Available discussions before filtering: {:?}",
                player_day, available_discussions
            );

            available_discussions = available_discussions
                .iter()
                .filter(|discussion| {
                    player_day >= discussion.day_system.min_day
                        && player_day <= discussion.day_system.max_day
                })
                .cloned()
                .collect::<Vec<_>>();

            if available_discussions.is_empty() {
                // If no discussions are available, exit loop
                info!(
                    "No available discussions found for player_day: {}",
                    player_day
                );
                break;
            }

            if available_discussions.is_empty() {
                break;
            }

            let percent_chance_vec = available_discussions
                .iter()
                .map(|discussion| discussion.percent_chance)
                .collect::<Vec<u16>>();

            let selected_index = get_random_index(&percent_chance_vec);
            let selected_discussion = available_discussions[selected_index].clone();

            let random_discussion_enum = select_random_discussion(selected_discussion.index);
            selected_discussions.push(random_discussion_enum);

            available_discussions.remove(selected_index);
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
                    cooldown: RANDOM_GRANDMA_1.day_system.cooldown,
                    max_day: RANDOM_GRANDMA_1.day_system.max_day,
                    min_day: RANDOM_GRANDMA_1.day_system.min_day,
                },
            },
            DailyDiscussionEnum::RandomGrandma2 => DailyDiscussion {
                id: RANDOM_GRANDMA_2.index,
                title: "Persistent Grandma".to_string(),
                description: "The same old lady insists on talking to you.".to_string(),
                image_atlas_index: 6,
                apparition_chance: RANDOM_GRANDMA_2.percent_chance,
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
                    cooldown: RANDOM_GRANDMA_2.day_system.cooldown,
                    max_day: RANDOM_GRANDMA_2.day_system.max_day,
                    min_day: RANDOM_GRANDMA_2.day_system.min_day,
                },
            },
            DailyDiscussionEnum::RandomGrandma3 => DailyDiscussion {
                id: RANDOM_GRANDMA_3.index,
                title: "Suspicious Grandma".to_string(),
                description: "The old lady seems to be hiding something.".to_string(),
                image_atlas_index: 7,
                apparition_chance: RANDOM_GRANDMA_3.percent_chance,
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
                    cooldown: RANDOM_GRANDMA_3.day_system.cooldown,
                    max_day: RANDOM_GRANDMA_3.day_system.max_day,
                    min_day: RANDOM_GRANDMA_3.day_system.min_day,
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
    pub id: u16,
    pub daily_event_type: DailyEventTypeEnum,
    pub day_system: DaySystem,
}

#[derive(Default, Debug, Component, Resource, Clone)]
pub struct DailyEvents(pub Vec<DailyEvent>);

impl DailyEvents {
    pub fn get_last_daily_event(&self) -> Option<&DailyEvent> {
        self.0.last()
    }

    // pub fn get_random_number_of_daily_events(&self, n: usize) -> Vec<DailyEvent> {
    //     let mut daily_events = Vec::new();
    //     let mut daily_discussion_number = 0;
    //     let mut daily_spontaneous_application_number = 0;

    //     // Discussion has a 66% chance of being selected.
    //     // Spontaneous application has a 33% chance of being selected.

    //     for _ in 0..n {
    //         let random_number = rand::thread_rng().gen_range(0..100);
    //         if random_number <= 100 {
    //             daily_discussion_number += 1;
    //         } else {
    //             daily_spontaneous_application_number += 1;
    //         }
    //     }

    //     let daily_discussion_enums =
    //         DailyDiscussionEnum::get_random_discussion_enums(daily_discussion_number);

    //     for daily_discussion_enum in daily_discussion_enums {
    //         let daily_discussion = DailyEvent {
    //             id: 0,
    //             daily_event_type: DailyEventTypeEnum::Discussion(daily_discussion_enum),
    //         };
    //         daily_events.push(daily_discussion);
    //     }

    //     let daily_sponaneous_applications =
    //         SpontaneousApplicationEnum::get_random_spontaneous_application_enums(
    //             daily_spontaneous_application_number,
    //         );

    //     for daily_spontaneous_application_enum in daily_sponaneous_applications {
    //         let daily_spontaneous_application = DailyEvent {
    //             id: 0,
    //             daily_event_type: DailyEventTypeEnum::SpontaneousApplication(
    //                 daily_spontaneous_application_enum,
    //             ),
    //         };
    //         daily_events.push(daily_spontaneous_application);
    //     }

    //     daily_events
    // }

    pub fn get_random_number_of_daily_events(
        &mut self,
        n: usize,
        player_day: u16,
    ) -> Vec<DailyEvent> {
        let mut daily_events = Vec::new();
        let mut daily_discussion_number = 0;
        let mut daily_spontaneous_application_number = 0;

        // Filter events by player day and select them based on their chance
        let filtered_discussions: Vec<DailyDiscussionEnum> =
            DailyDiscussionEnum::get_random_discussion_enums(n, player_day);

        for discussion_enum in filtered_discussions {
            let mut daily_discussion = discussion_enum.get_daily_discussion();
            daily_discussion.day_system.min_day += daily_discussion.day_system.cooldown; // Update min_day
            daily_events.push(DailyEvent {
                id: 0,
                daily_event_type: DailyEventTypeEnum::Discussion(discussion_enum),
                day_system: daily_discussion.day_system.clone(),
            });
        }

        let filtered_spontaneous_applications: Vec<SpontaneousApplicationEnum> =
            SpontaneousApplicationEnum::get_random_spontaneous_application_enums(
                daily_spontaneous_application_number,
                player_day,
            );

        for spontaneous_application_enum in filtered_spontaneous_applications {
            let mut spontaneous_application =
                spontaneous_application_enum.get_spontaneous_application();
            spontaneous_application.day_system.min_day +=
                spontaneous_application.day_system.cooldown; // Update min_day
            daily_events.push(DailyEvent {
                id: 0,
                daily_event_type: DailyEventTypeEnum::SpontaneousApplication(
                    spontaneous_application_enum,
                ),
                day_system: spontaneous_application.day_system.clone(),
            });
        }

        daily_events
    }
}
