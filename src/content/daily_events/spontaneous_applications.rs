use crate::structs::daily_events_folder::{
    daily_events::DaySystem, spontaneous_applications::SpontaneousApplication,
};
use bevy::prelude::*;

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub enum SpontaneousApplicationEnum {
    RandomNoob1,
    RandomNoob2,
}

pub fn select_random_spontaneous_application(index: u16) -> SpontaneousApplicationEnum {
    match index {
        1 => SpontaneousApplicationEnum::RandomNoob1,
        2 => SpontaneousApplicationEnum::RandomNoob2,
        // Should never happen
        _ => SpontaneousApplicationEnum::RandomNoob1,
    }
}

pub fn get_spontaneous_application(
    spontaneous_application_enum: &SpontaneousApplicationEnum,
) -> SpontaneousApplication {
    match spontaneous_application_enum {
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
