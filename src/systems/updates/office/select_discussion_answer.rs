use crate::structs::{
    daily_events_folder::{
        daily_events::DailyEvents,
        discussions::{Answer, DailyDiscussion},
        spontaneous_applications::SpontaneousApplication,
    },
    general_structs::DailyEventsModalVisible,
};
use bevy::prelude::*;

pub fn select_discussion_answer(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Answer,
            Option<&DailyDiscussion>,
            Option<&SpontaneousApplication>,
        ),
        (Changed<Interaction>, With<Answer>),
    >,
    mut windows: Query<&mut Window>,
    mut daily_events: ResMut<DailyEvents>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
) {
    let mut window = windows.single_mut();
    for (interaction, mut background_color, answer, discussion, spontaneous_application) in
        query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                // daily_events.remove_daily_discussion_by_id(discussion.id);

                // Check if there's a DailyDiscussion or SpontaneousApplication and remove it accordingly
                if let Some(discussion) = discussion {
                    daily_events.remove_daily_discussion_by_id(discussion.id);
                } else if let Some(application) = spontaneous_application {
                    daily_events.remove_spontaneous_application_by_id(application.id);
                }

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
