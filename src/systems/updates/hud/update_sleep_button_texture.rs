use crate::structs::{
    general_structs::{DayTime, EVENING_TIME, MIDDAY_TIME},
    trigger_structs::SleepButtonTrigger,
};
use bevy::prelude::*;

pub fn update_sleep_button_texture(
    day_time: Res<DayTime>,
    mut query: Query<(&mut UiImage, &SleepButtonTrigger)>,
) {
    for (mut ui_image, _) in query.iter_mut() {
        let second_count = match day_time.second_count {
            0..MIDDAY_TIME => 0.,
            MIDDAY_TIME..EVENING_TIME => 1.,
            EVENING_TIME.. => 2.,
        };

        // Update the texture atlas index
        if let Some(texture_atlas) = ui_image.texture_atlas.as_mut() {
            texture_atlas.index = second_count as usize;
        }
    }
}
