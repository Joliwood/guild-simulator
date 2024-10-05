use crate::structs::{
    general_structs::MissionReportsModalVisible,
    missions::{MissionReports, Missions},
    trigger_structs::MissionReportModalContentTrigger,
};
use bevy::prelude::*;

pub fn mission_report_modal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MissionReportModalContentTrigger>>,
    mission_reports_modal_visibility: Res<MissionReportsModalVisible>,
    mission_reports: Res<MissionReports>,
    missions: Res<Missions>,
) {
    // Despawn existing modals
    if mission_reports_modal_visibility.is_changed() && !mission_reports_modal_visibility.0 {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let mission_reports_len = mission_reports.0.len();

    if mission_reports_modal_visibility.is_changed()
        && mission_reports_modal_visibility.0
        && mission_reports_len > 0
    {
        let container_image: Handle<Image> =
            asset_server.load("images/rooms/barrack/inventory_container.png");

        let last_mission_report = mission_reports.get_last_mission_report();
        if last_mission_report.is_none() {
            return;
        }

        let mission = missions.get_mission_by_id(last_mission_report.unwrap().mission_id);
        if mission.is_none() {
            return;
        }

        info!("=====> mission: {:?}", mission);

        commands
            .spawn(ImageBundle {
                image: container_image.into(),
                style: Style {
                    width: Val::Px(300.),
                    height: Val::Px(450.),
                    display: Display::Flex,
                    justify_self: JustifySelf::Center,
                    position_type: PositionType::Absolute,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    top: Val::Px(155.),
                    ..default()
                },
                z_index: ZIndex::Global(-1),
                ..default()
            })
            .insert(MissionReportModalContentTrigger)
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        mission.unwrap().name,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..Default::default()
                });
            });
        // });
        // }
    }
}
