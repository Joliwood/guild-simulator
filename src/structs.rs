// #![allow(dead_code)]

use crate::enums::{RecruitEnum, RoomEnum};
use bevy::prelude::{Component, Resource};
use uuid::Uuid;

// --- Triggers --- //

#[derive(Resource, Component)]
pub struct GoldCountTrigger;

#[derive(Component)]
pub struct PlayerStatsRoomTrigger;

#[derive(Component)]
pub struct ResetRoomTrigger;

#[derive(Component)]
pub struct PlayerStatsRecruitsTrigger;

#[derive(Component)]
pub struct SelectedRecruitTrigger;

#[derive(Component)]
pub struct SelectedMissionTrigger;

#[derive(Component)]
pub struct ModalContentTrigger;

// --- Definition of structs --- //

#[derive(Component, Resource)]
pub struct MissionModalVisible(pub bool);

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Component, Resource)]
pub struct PlayerStats {
    pub golds: i32,
    pub recruits: Vec<RecruitStats>,
    pub room: RoomEnum,
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct RecruitStats {
    pub id: Uuid,
    pub class: RecruitEnum,
    pub endurance: u16,
    pub experience: u32,
    pub intelligence: u16,
    pub level: u8,
    pub max_experience: u32,
    pub strength: u16,
}

#[derive(Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedRecruit(pub Option<RecruitStats>);

#[derive(Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedMission {
    pub mission: Option<Mission>,
    pub recruit_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
    pub name: String,
    pub endurance: u16,
    pub experience: u32,
    pub intelligence: u16,
    pub level: u8,
    pub strength: u16,
}

#[derive(Component, Resource)]
pub struct Missions(pub Vec<Mission>);
#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub level: u8,
    pub ennemy: Ennemy,
}

// --- Implementations --- //

impl Default for SelectedRecruit {
    fn default() -> Self {
        Self(None)
    }
}

impl Default for SelectedMission {
    fn default() -> Self {
        Self {
            mission: None,
            recruit_id: None,
        }
    }
}

impl PlayerStats {
    pub fn increment_golds(&mut self, amount: i32) {
        self.golds += amount;
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            golds: 0,
            room: RoomEnum::CommandRoom,
            recruits: vec![],
        }
    }
}

impl Default for Missions {
    fn default() -> Self {
        Self(vec![
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 1".to_string(),
                level: 1,
                ennemy: Ennemy {
                    name: "Ennemy 1".to_string(),
                    endurance: 10,
                    experience: 0,
                    intelligence: 5,
                    level: 1,
                    strength: 10,
                },
            },
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 2".to_string(),
                level: 2,
                ennemy: Ennemy {
                    name: "Ennemy 2".to_string(),
                    endurance: 15,
                    experience: 0,
                    intelligence: 7,
                    level: 2,
                    strength: 15,
                },
            },
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 3".to_string(),
                level: 3,
                ennemy: Ennemy {
                    name: "Ennemy 3".to_string(),
                    endurance: 20,
                    experience: 0,
                    intelligence: 10,
                    level: 3,
                    strength: 20,
                },
            },
        ])
    }
}
