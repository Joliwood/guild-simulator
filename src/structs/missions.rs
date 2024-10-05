use bevy::prelude::*;
use uuid::Uuid;

use super::general_structs::Ennemy;

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct MissionReport {
    pub recruit_id: Uuid,
    pub mission_id: Uuid,
    pub success: bool,
    pub experience_gained: Option<u32>,
    pub golds_gained: Option<i32>,
}

#[derive(Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct SelectedMission {
    pub mission: Option<Mission>,
    pub percent_of_victory: Option<u32>,
    pub recruit_id: Option<Uuid>,
}

impl SelectedMission {
    pub fn get_mission(&self) -> Option<Mission> {
        if let Some(mission) = &self.mission {
            return Some(mission.clone());
        }

        None
    }
}

#[derive(Debug, Component, Resource)]
pub struct Missions(pub Vec<Mission>);

impl Missions {
    pub fn get_mission_by_id(&self, id: Uuid) -> Option<Mission> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == id) {
            return Some(mission.clone());
        }
        None
    }

    pub fn assign_recruit_id_to_mission(&mut self, mission_id: Uuid, recruit_id: Uuid) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.assign_recruit_by_id(recruit_id);
        }
    }

    pub fn decrement_days_left_by_mission_id(&mut self, mission_id: Uuid) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.decrement_days_left();
        }
    }

    pub fn attribute_days_left_to_mission(&mut self, mission_id: Uuid) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.attribute_days_left();
        }
    }

    pub fn is_mission_over(&self, mission_id: Uuid) -> bool {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.days_left.is_none();
        }
        false
    }

    pub fn get_recruit_id_send_by_mission_id(&self, mission_id: Uuid) -> Option<Uuid> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.recruit_send;
        }
        None
    }

    pub fn desassign_recruit_to_mission(&mut self, mission_id: Uuid) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.desassign_recruit();
        }
    }

    pub fn get_mission_enemmy_level_by_id(&self, mission_id: Uuid) -> Option<u8> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return Some(mission.ennemy.level);
        }
        None
    }
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Mission {
    pub days: u8,
    pub ennemy: Ennemy,
    pub id: Uuid,
    pub level: u8,
    pub name: String,
    pub recruit_send: Option<Uuid>,
    pub days_left: Option<u8>,
}

impl Mission {
    pub fn decrement_days_left(&mut self) {
        if let Some(days_left) = &mut self.days_left {
            if *days_left == 1 {
                return self.days_left = None;
            }
            if *days_left > 0 {
                return *days_left -= 1;
            }
        }
    }

    pub fn assign_recruit_by_id(&mut self, recruit_id: Uuid) {
        self.recruit_send = Some(recruit_id);
    }

    pub fn desassign_recruit(&mut self) {
        self.recruit_send = None;
    }

    pub fn attribute_days_left(&mut self) {
        self.days_left = Some(self.days);
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

impl Default for Missions {
    fn default() -> Self {
        Self(vec![
            Mission {
                days_left: None,
                days: 1,
                id: Uuid::new_v4(),
                level: 1,
                name: "Mission 1".to_string(),
                recruit_send: None,
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
                days_left: None,
                days: 1,
                id: Uuid::new_v4(),
                level: 2,
                name: "Mission 2".to_string(),
                recruit_send: None,
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
                days_left: Some(1),
                days: 2,
                id: Uuid::new_v4(),
                level: 3,
                name: "Mission 3".to_string(),
                recruit_send: None,
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
