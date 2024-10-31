use crate::{
    audio::play_sound::play_sound,
    enums::SoundEnum,
    my_assets::MyAssets,
    structs::{general_structs::MissionReportsModalVisible, trigger_structs::MissionReportTrigger},
};
use bevy::prelude::*;

pub fn toggle_mission_reports(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<MissionReportTrigger>)>,
    mut windows: Query<&mut Window>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
) {
    let mut window = windows.single_mut();

    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                mission_reports_modal_visibility.0 = true;
                play_sound(&my_assets, &mut commands, SoundEnum::PaperTouch);
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
