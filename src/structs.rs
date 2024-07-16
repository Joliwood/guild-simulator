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

// --- Definition of structs --- //

#[derive(Component, Resource)]
pub struct ModalVisible(pub bool);

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
pub struct SelectedMission(pub Option<Mission>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
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
        Self(None)
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
                level: 1,
                ennemy: Ennemy {
                    endurance: 10,
                    experience: 0,
                    intelligence: 5,
                    level: 1,
                    strength: 10,
                },
            },
            Mission {
                level: 2,
                ennemy: Ennemy {
                    endurance: 15,
                    experience: 0,
                    intelligence: 7,
                    level: 2,
                    strength: 15,
                },
            },
            Mission {
                level: 3,
                ennemy: Ennemy {
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
