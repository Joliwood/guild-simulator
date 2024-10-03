use bevy::prelude::{Component, Resource};

#[derive(Resource, Component)]
pub struct GoldCountTrigger;

#[derive(Component)]
pub struct PlayerStatsRoomTrigger;

#[derive(Component)]
pub struct ResetRoomTrigger;

#[derive(Component)]
pub struct SelectedRecruitTrigger;

#[derive(Component)]
pub struct SelectedMissionRecruitIdTrigger;

#[derive(Component)]
pub struct SelectedMissionPercentOfVictoryTrigger;

#[derive(Component)]
pub struct ModalContentTrigger;

#[derive(Component)]
pub struct MissionNotificationTrigger;

#[derive(Debug, Component)]
pub struct NotificationToastTrigger;

#[derive(Debug, Component)]
pub struct SleepButtonTrigger;
