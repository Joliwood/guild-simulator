use bevy::prelude::{Component, Resource};

#[derive(Resource, Component)]
pub struct GoldCountTrigger;

#[derive(Component)]
pub struct PlayerStatsRoomTrigger;

#[derive(Component)]
pub struct ResetRoomTrigger;

#[derive(Component)]
pub struct SelectedMissionRecruitIdTrigger;

#[derive(Component)]
pub struct SelectedMissionPercentOfVictoryTrigger;

#[derive(Component)]
pub struct MissionModalContentTrigger;

#[derive(Component)]
pub struct MissionNotificationTrigger;

#[derive(Debug, Component)]
pub struct NotificationToastTrigger;

#[derive(Debug, Component)]
pub struct SleepButtonTrigger;

#[derive(Debug, Component)]
pub struct MissionReport;

#[derive(Component)]
pub struct MissionReportModalContentTrigger;

#[derive(Component)]
pub struct MissionReportButtonTrigger;

#[derive(Component)]
pub struct MissionReportModalSignButtonTrigger;
