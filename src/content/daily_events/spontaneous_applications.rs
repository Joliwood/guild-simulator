use crate::structs::daily_events_folder::{
    daily_events::DaySystem, spontaneous_applications::SpontaneousApplication,
};

// --- To update whenever the content is updated --- //
const MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER: u16 = 2;

pub fn get_all_spontaneous_applications() -> Vec<SpontaneousApplication> {
    (1..=MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER)
        .map(|i| get_spontaneous_application(&i))
        .collect()
}

pub fn get_spontaneous_application(spontaneous_application_index: &u16) -> SpontaneousApplication {
    match spontaneous_application_index {
        1 => SpontaneousApplication {
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
        2 => SpontaneousApplication {
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
        _ => panic!(
            "Spontaneous application index not found: {}",
            spontaneous_application_index
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::*;
    use std::panic;

    #[test]
    fn test_get_spontaneous_application_should_panic() {
        let result = panic::catch_unwind(|| {
            get_spontaneous_application(&(MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER + 1));
        });

        assert!(
            result.is_err(),
            "{}",
            "When you update the content, you have to update also the MAX_DAILY_SPONTANEOUS_APPLICATION_NUMBER constant".red()
        );
    }
}
