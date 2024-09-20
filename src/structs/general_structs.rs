use crate::enums::{RecruitEnum, RoomEnum};
use bevy::prelude::{Component, Resource};
use uuid::Uuid;

#[derive(Component, Resource)]
pub struct MissionModalVisible(pub bool);

#[derive(Component)]
pub struct UniqueId(pub String);

#[derive(Component, Resource)]
pub struct PlayerStats {
    pub experience: u32,
    pub golds: i32,
    pub guild_level: i8,
    pub max_experience: u32,
    pub recruits: Vec<RecruitStats>,
    pub room: RoomEnum,
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct RecruitStats {
    pub class: RecruitEnum,
    pub endurance: u16,
    pub experience: u32,
    pub id: Uuid,
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
    pub percent_of_victory: Option<u32>,
    pub recruit_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Resource)]
pub struct Ennemy {
    pub endurance: u16,
    pub experience: u32,
    pub intelligence: u16,
    pub level: u8,
    pub name: String,
    pub strength: u16,
}

#[derive(Component, Resource)]
pub struct Missions(pub Vec<Mission>);
#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Mission {
    pub ennemy: Ennemy,
    pub id: Uuid,
    pub level: u8,
    pub name: String,
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
            percent_of_victory: None,
            recruit_id: None,
        }
    }
}

impl PlayerStats {
    pub fn increment_golds(&mut self, amount: i32) {
        self.golds += amount;
    }

    fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        // Reset the experience with left experience after leveling up
        // Then level up
        if self.experience >= self.max_experience {
            self.experience -= self.max_experience;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.guild_level += 1;
        // Set the max experience to the current experience * 2
        self.max_experience *= 2;
    }
}

impl RecruitStats {
    pub fn gain_xp(&mut self, xp: u32) {
        self.experience += xp;

        // Reset the experience with left experience after leveling up
        // Then level up
        if self.experience >= self.max_experience {
            self.experience -= self.max_experience;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        // Set the max experience to the current experience * 2
        self.max_experience *= 2;
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            experience: 0,
            golds: 0,
            guild_level: 1,
            max_experience: 100,
            recruits: vec![],
            room: RoomEnum::CommandRoom,
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
                    endurance: 10,
                    experience: 0,
                    intelligence: 5,
                    level: 1,
                    name: "Ennemy 1".to_string(),
                    strength: 10,
                },
            },
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 2".to_string(),
                level: 2,
                ennemy: Ennemy {
                    endurance: 15,
                    experience: 0,
                    intelligence: 7,
                    level: 2,
                    name: "Ennemy 2".to_string(),
                    strength: 15,
                },
            },
            Mission {
                id: Uuid::new_v4(),
                name: "Mission 3".to_string(),
                level: 3,
                ennemy: Ennemy {
                    endurance: 20,
                    experience: 0,
                    intelligence: 10,
                    level: 3,
                    name: "Ennemy 3".to_string(),
                    strength: 20,
                },
            },
        ])
    }
}
