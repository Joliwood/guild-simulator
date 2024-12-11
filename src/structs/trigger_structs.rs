use crate::enums::RoomEnum;
use bevy::prelude::{Component, Resource};

use super::equipments::ItemEnum;

#[derive(Resource, Component)]
pub struct GoldCountTrigger;

#[derive(Component)]
pub struct ResetRoomTrigger;

#[derive(Component)]
pub struct MissionModalContentTrigger;

#[derive(Debug, Component)]
pub struct NotificationToastTrigger;

#[derive(Debug, Component)]
pub struct SleepButtonTrigger;

#[derive(Component)]
pub struct MissionReportModalContentTrigger;

#[derive(Component)]
pub struct MissionReportModalSignButtonTrigger;

#[derive(Component)]
pub struct DailyEventTrigger;

#[derive(Component)]
pub struct DailyEventTextTrigger;

#[derive(Component)]
pub struct SelectAnswerTrigger;

#[derive(Component)]
pub struct GuildLvlTrigger;

#[derive(Component)]
pub struct PlayerDayTrigger;

#[derive(Component)]
pub struct RecruitCountTrigger;

#[derive(Component)]
pub struct ReputationCountTrigger;

#[derive(Component)]
pub struct ToxicityCountTrigger;

#[derive(Component)]
pub struct RoomButtonTrigger(pub RoomEnum);

#[derive(Component)]
pub struct SelectMapTrigger;

#[derive(Component)]
pub struct RealTimeDayProgressBarTrigger;

#[derive(Component)]
pub struct CloseMissionModalTrigger;

#[derive(Component)]
pub struct ItemInInventoryTrigger(pub Option<ItemEnum>);

#[derive(Component)]
pub struct CommandRoomNotificationContainerTrigger;

#[derive(Component)]
pub struct CommandRoomNotificationTrigger;

#[derive(Component)]
pub struct OfficeRoomNotificationContainerTrigger;

#[derive(Component)]
pub struct OfficeRoomNotificationTrigger;

#[derive(Component)]
pub struct BarrackRoomNotificationContainerTrigger;

#[derive(Component)]
pub struct BarrackRoomNotificationTrigger;

#[derive(Component)]
pub struct TutoMessageModalTrigger;

#[derive(Component)]
pub struct TutoDoneCloseModalTrigger;

#[derive(Component)]
pub struct CloseTutoMessageTrigger;

#[derive(Component)]
pub struct SkipTutoMessageTrigger;
