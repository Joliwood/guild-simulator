use bevy::prelude::*;

use crate::{
    structs::{
        general_structs::MissionReportsModalVisible, missions::MissionReports,
        trigger_structs::MissionReportModalSignButtonTrigger,
    },
    systems::systems_constants::HOVERED_BUTTON,
};

pub fn sign_mission_report(
    // mut query: Query<
    //     (&Interaction, &mut Style, &mut BackgroundColor),
    //     (
    //         Changed<Interaction>,
    //         With<MissionReportModalSignButtonTrigger>,
    //     ),
    // >,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Style,
            &MissionReportModalSignButtonTrigger,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mission_reports: Res<MissionReports>,
) {
    let mut window = windows.single_mut();

    // for (interaction, mut style, mut color) in query.iter_mut() {
    for (interaction, mut style, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                info!("hello")
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                // *color = HOVERED_BUTTON.into();
                // Add a border when hovered
                style.border = UiRect {
                    left: Val::Px(3.0),
                    right: Val::Px(3.0),
                    top: Val::Px(3.0),
                    bottom: Val::Px(3.0),
                };
                // WIP
                // BorderColor(Color::WHITE);
            }
            Interaction::None => {
                // Remove the border when not interacted with
                window.cursor.icon = CursorIcon::Default;
                // *color = Color::NONE.into();
                style.border = UiRect::default();
            }
        }
    }
}
