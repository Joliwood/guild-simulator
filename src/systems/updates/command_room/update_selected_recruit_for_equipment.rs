use crate::structs::{
    missions::SelectedMission,
    trigger_structs::{SelectedMissionPercentOfVictoryTrigger, SelectedMissionRecruitIdTrigger},
};
use bevy::prelude::{DetectChanges, Entity, Res, Single, Text, TextUiWriter, With};

pub fn update_selected_mission_recruit_id(
    selected_mission: Res<SelectedMission>,
    query: Single<Entity, (With<SelectedMissionRecruitIdTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if selected_mission.is_changed() {
        *writer.text(*query, 0) = format!("{:?}", selected_mission.recruit_id);
    }
}

pub fn update_update_selected_mission_percentage_of_victory(
    selected_mission: Res<SelectedMission>,
    query: Single<Entity, (With<SelectedMissionPercentOfVictoryTrigger>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if selected_mission.is_changed() {
        *writer.text(*query, 0) = format!("{:?} %", selected_mission.percent_of_victory);
    }
}
