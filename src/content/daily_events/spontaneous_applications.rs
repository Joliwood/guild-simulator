use crate::structs::daily_events::{DaySystem, SpontaneousApplication, SpontaneousApplicationEnum};

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
