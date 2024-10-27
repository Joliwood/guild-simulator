use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    my_assets::MyAssets,
    structs::{general_structs::DailyEventsModalVisible, trigger_structs::DailyEventTrigger},
};
use bevy::prelude::*;

pub fn toggle_daily_event_documents(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<DailyEventTrigger>)>,
    mut windows: Query<&mut Window>,
    mut daily_events_modal_visibility: ResMut<DailyEventsModalVisible>,
) {
    let mut window = windows.single_mut();

    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                daily_events_modal_visibility.0 = true;
                play_sound(&my_assets, &mut commands, SoundEnum::PickingGolds);
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}
