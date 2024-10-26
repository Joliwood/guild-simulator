#![allow(dead_code)]
use bevy::prelude::*;

use super::player_stats::PlayerStats;
use rand::Rng;

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
    pub fn get_discussion(&self) -> DailyDiscussion {
        match self {
            DailyDiscussionEnum::RandomGrandma1 => DailyDiscussion {
                id: 1,
                title: "Curious Grandma".to_string(),
                description: "An old lady approaches with a question about your guild.".to_string(),
                image_atlas_index: 5,
                apparition_chance: 99,
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
                apparition_chance: 50,
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
                apparition_chance: 99,
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
                discussion_enum.get_discussion().apparition_chance
            }
        }
    }
}

#[derive(Debug, Component, Resource, Clone)]
pub struct DailyEvents(pub Vec<DailyEvent>);

impl Default for DailyEvents {
    fn default() -> Self {
        Self(vec![DailyEvent {
            id: 1,
            daily_event_type: DailyEventTypeEnum::Discussion(DailyDiscussionEnum::RandomGrandma1),
        }])
    }
}

// impl DailyEvents {
//     pub fn get_all_percent_chance(&self) -> u16 {
//         let azpo = self.0.iter().map(|event| event.get_percent_chance()).sum();
//         info!("All percent chance : {:?}", azpo);
//         azpo
//     }

//     pub fn get_one_random_event_based_on_all_percent_chance(&self) -> Option<DailyEvent> {
//         let all_percent_chance = self.get_all_percent_chance();
//         let random_percent = rand::random::<u16>() % all_percent_chance;
//         let mut current_percent = 0;
//         for event in self.0.iter() {
//             let percent_chance = event.get_percent_chance();
//             if random_percent < current_percent + percent_chance {
//                 info!("-------- Random event : {:?}", event);
//                 return Some(event.clone());
//             }
//             current_percent += percent_chance;
//         }
//         None
//     }

//     pub fn add_impact_to_player_stats(
//         &self,
//         player_stats: &mut PlayerStats,
//         gold_impact: Option<i32>,
//         experience_impact: Option<u32>,
//         toxicity_impact: Option<i8>,
//     ) {
//         if let Some(gold_impact) = gold_impact {
//             player_stats.increment_golds(gold_impact);
//         }
//         if let Some(experience_impact) = experience_impact {
//             player_stats.gain_xp(experience_impact);
//         }
//         if let Some(toxicity_impact) = toxicity_impact {
//             player_stats.gain_toxitiy(toxicity_impact);
//         }
//     }

//     pub fn remove_event_by_id(&mut self, id: u16) {
//         self.0.retain(|event| event.id != id);
//     }
// }

impl DailyEvents {
    pub fn select_random_discussions(&self) -> Vec<DailyEvent> {
        let mut rng = rand::thread_rng();
        let mut selected_events = Vec::new();

        // 1. Sélectionne la première discussion obligatoire
        if let Some(first_event) = self.select_one_event_based_on_chance() {
            selected_events.push(first_event);

            // 2. Sélectionne une deuxième discussion optionnelle
            if rng.gen_bool(0.5) {
                // 50% de chance d'avoir une deuxième discussion
                if let Some(second_event) =
                    self.select_one_event_based_on_chance_except(&selected_events)
                {
                    selected_events.push(second_event);
                }
            }
        }

        selected_events
    }

    // Fonction utilitaire pour sélectionner une discussion
    pub fn select_one_event_based_on_chance(&self) -> Option<DailyEvent> {
        // let total_chance: u16 = self.0.iter().map(|event| event.get_percent_chance()).sum();
        let total_change: u16 = 50 + 99 + 99;
        let random_number = rand::thread_rng().gen_range(0..total_change);
        if random_number < 50 {
            return Some(DailyEvent {
                id: 1,
                daily_event_type: DailyEventTypeEnum::Discussion(
                    DailyDiscussionEnum::RandomGrandma1,
                ),
            });
        } else if random_number < 50 + 99 {
            return Some(DailyEvent {
                id: 2,
                daily_event_type: DailyEventTypeEnum::Discussion(
                    DailyDiscussionEnum::RandomGrandma2,
                ),
            });
        } else {
            return Some(DailyEvent {
                id: 3,
                daily_event_type: DailyEventTypeEnum::Discussion(
                    DailyDiscussionEnum::RandomGrandma3,
                ),
            });
        }
    }

    // Sélectionner une discussion en évitant celles déjà sélectionnées
    fn select_one_event_based_on_chance_except(
        &self,
        exclusions: &[DailyEvent],
    ) -> Option<DailyEvent> {
        let filtered_events: Vec<_> = self
            .0
            .iter()
            .filter(|event| !exclusions.contains(event))
            .cloned()
            .collect();

        let mut random_events = DailyEvents(filtered_events);
        random_events.select_one_event_based_on_chance()
    }
}
