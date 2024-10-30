use crate::structs::{
    daily_events_folder::{
        daily_events::DailyEvents,
        discussions::{Answer, DailyDiscussion},
    },
    general_structs::DailyEventsModalVisible,
    player_stats::PlayerStats,
};
use bevy::prelude::*;

pub fn select_discussion_answer(
    mut query: Query<
        (
            &Interaction,
            &mut Style,
            &mut BackgroundColor,
            &Answer,
            &DailyDiscussion,
        ),
        (Changed<Interaction>, With<Answer>),
    >,
    mut windows: Query<&mut Window>,
    mut daily_events: ResMut<DailyEvents>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
) {
    let mut window = windows.single_mut();
    for (interaction, mut color, mut background_color, answer, discussion) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                daily_events.remove_daily_discussion_by_id(discussion.id);

                daily_events_modal_visibility.0 = false;

                if !daily_events.0.is_empty() {
                    daily_events_modal_visibility.0 = true;
                }
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                background_color.0 = Color::srgba(0., 0., 0., 0.4);
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Default;
                background_color.0 = Color::srgba(0., 0., 0., 0.5);
            }
        };
    }
}
