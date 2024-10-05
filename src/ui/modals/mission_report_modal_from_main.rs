use bevy::prelude::*;

use crate::{
    enums::ColorPaletteEnum,
    structs::{
        general_structs::MissionReportsModalVisible,
        trigger_structs::MissionReportModalContentTrigger,
    },
};

pub fn mission_report_modal_from_main(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MissionReportModalContentTrigger>>,
    // mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mission_reports_modal_visibility: Res<MissionReportsModalVisible>,
) {
    // Despawn existing modals
    if mission_reports_modal_visibility.is_changed() && !mission_reports_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // if mission_reports_modal_visibility.is_changed() && mission_reports_modal_visibility.0 {

    // info!(
    //     "=====> MISSION REPORT MODAL VISIBILITY: {}",
    //     mission_reports_modal_visibility.0
    // );

    // if mission_reports_modal_visibility.0 == true {
    if mission_reports_modal_visibility.is_changed() && mission_reports_modal_visibility.0 {
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                // background_color: if mission_reports_modal_visibility.0 {
                //     BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8))
                // } else {
                //     BackgroundColor(Color::NONE)
                // },
                ..default()
            })
            .insert(MissionReportModalContentTrigger);
        // }
    }
}
