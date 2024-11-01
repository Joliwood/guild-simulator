use crate::{
    audio::play_sound::play_sound,
    enums::{ColorPaletteEnum, RecruitStateEnum, SoundEnum},
    my_assets::MyAssets,
    structs::{
        general_structs::MissionReportsModalVisible,
        maps::Maps,
        missions::{MissionReport, MissionReports, Missions},
        player_stats::PlayerStats,
        trigger_structs::MissionReportModalSignButtonTrigger,
    },
    systems::systems_constants::HOVERED_BUTTON,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn sign_mission_report(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Style,
            &MissionReportModalSignButtonTrigger,
            &mut BackgroundColor,
            &MissionReport,
        ),
        Changed<Interaction>,
    >,
    mut windows: Query<&mut Window>,
    mut mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    mut player_stats: ResMut<PlayerStats>,
    mut missions: ResMut<Missions>,
    mut mission_reports: ResMut<MissionReports>,
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    mut maps: ResMut<Maps>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut style, _button, mut color, mission_report) in interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                player_stats.update_state_of_recruit(
                    mission_report.recruit_id,
                    RecruitStateEnum::Available,
                );

                missions.desassign_recruit_to_mission(mission_report.mission_id);

                if mission_report.success {
                    player_stats.gain_xp(mission_report.experience_gained.unwrap());
                    player_stats.gain_xp_to_recruit(
                        mission_report.recruit_id,
                        mission_report.experience_gained.unwrap(),
                    );
                    player_stats.increment_golds(mission_report.golds_gained.unwrap());
                    play_sound(&my_assets, &mut commands, SoundEnum::PickingGolds);

                    missions.unlock_missions_by_mission_id(mission_report.mission_id);

                    for loot in mission_report.loots.iter() {
                        player_stats.add_item(loot.clone());
                    }
                    player_stats.stats.golds_earned += mission_report.golds_gained.unwrap();
                    player_stats.stats.mission_completed += 1;

                    let map_id = missions.get_map_id_by_mission_id(mission_report.mission_id);
                    let map = maps.get_map_by_optional_id(map_id);
                    if map.is_some() {
                        maps.finish_mission_by_id(mission_report.mission_id);
                    }
                } else {
                    let random_number_from_0_to_100 = rand::random::<u8>();
                    if random_number_from_0_to_100 > 80 {
                        // The recruit die
                        player_stats.remove_recruit_by_id(mission_report.recruit_id);
                        play_sound(&my_assets, &mut commands, SoundEnum::DeadMale);
                    }
                }

                mission_reports.remove_mission_report_by_id(mission_report.mission_id);

                mission_reports_modal_visibility.0 = false;

                play_sound(&my_assets, &mut commands, SoundEnum::PencilSign);

                if !mission_reports.0.is_empty() {
                    mission_reports_modal_visibility.0 = true;
                }
            }
            Interaction::Hovered => {
                window.cursor.icon = CursorIcon::Pointer;
                *color = HOVERED_BUTTON.into();
                // Add a border when hovered
                style.border = UiRect {
                    left: Val::Px(3.0),
                    right: Val::Px(3.0),
                    top: Val::Px(3.0),
                    bottom: Val::Px(3.0),
                };
            }
            Interaction::None => {
                // Remove the border when not interacted with
                window.cursor.icon = CursorIcon::Default;
                *color = BackgroundColor(ColorPaletteEnum::DarkBrown.as_color());
                style.border = UiRect::default();
            }
        }
    }
}
