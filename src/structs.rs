#![allow(dead_code)]

use crate::enums::{RecruitEnum, RoomEnum};
use bevy::{
    prelude::{Component, Event, Resource},
    state::state::States,
};
use uuid::Uuid;

// --- Triggers --- //

#[derive(Component)]
pub struct GoldCountTrigger;

#[derive(Component)]
pub struct PlayerStatsRoomTrigger;

#[derive(Component)]
pub struct ResetRoomTrigger;

#[derive(Component)]
pub struct PlayerStatsRecruitsTrigger;

#[derive(Component)]
pub struct RecruitsTrigger;

// --- Definition of structs --- //

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

#[derive(Resource, Debug, Event)]
pub struct SelectRecruitEvent(pub Uuid);

impl Default for SelectedRecruit {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Component)]
pub struct SelectedRecruitTrigger;

// --- Implementations --- //

impl PlayerStats {
    pub fn increment_golds(&mut self, amount: i32) {
        println!(
            "Incrementing golds by {} for a total of : {}",
            amount, self.golds,
        );
        self.golds += amount;
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            golds: 5,
            room: RoomEnum::Barrack,
            recruits: vec![],
        }
    }
}
