use bevy::prelude::*;
use uuid::Uuid;

use crate::utils::{get_global_points, get_victory_percentage};

use super::{
    general_structs::Ennemy,
    player_stats::{self, PlayerStats},
};

#[derive(Default, Debug, Component, Resource)]
pub struct MissionReports(pub Vec<MissionReport>);

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct MissionReport {
    pub recruit_id: Uuid,
    pub mission_id: u16,
    pub success: bool,
    pub experience_gained: Option<u32>,
    pub golds_gained: Option<i32>,
    pub percent_of_victory: u32,
}

impl MissionReports {
    pub fn add_mission_report(&mut self, report: MissionReport) {
        self.0.push(report);
    }

    pub fn get_mission_report_by_id(&self, mission_id: u16) -> Option<MissionReport> {
        if let Some(report) = self.0.iter().find(|report| report.mission_id == mission_id) {
            return Some(report.clone());
        }
        None
    }

    pub fn get_last_mission_report(&self) -> Option<MissionReport> {
        if let Some(report) = self.0.last() {
            return Some(report.clone());
        }
        None
    }

    pub fn remove_mission_report_by_id(&mut self, mission_id: u16) {
        if let Some(index) = self
            .0
            .iter()
            .position(|report| report.mission_id == mission_id)
        {
            self.0.remove(index);
        }
    }
}

#[derive(Default, Resource, Debug, Component, Clone, Eq, PartialEq, Hash)]
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

    pub fn calculate_percent_of_victory(&mut self, player_stats: &Res<PlayerStats>) {
        if self.mission.is_none() {
            return;
        }

        let ennemy = self.mission.as_ref().unwrap().ennemy.clone();
        let ennemy_global_points =
            get_global_points(ennemy.strength, ennemy.endurance, ennemy.intelligence);

        let recruit_id = self.recruit_id.unwrap();
        let recruit = player_stats.get_recruit_by_id(recruit_id).unwrap();
        let recruit_global_points = recruit.get_total_merged_stats();

        let victory_percentage =
            get_victory_percentage(recruit_global_points as u16, ennemy_global_points) as u32;

        self.percent_of_victory = Some(victory_percentage);

        // let recruit = self.percent_of_victory = Some(get_victory_percentage(
        //     recruit_global_points,
        //     ennemy_global_points,
        // ));
    }
}

#[derive(Debug, Component, Resource)]
pub struct Missions(pub Vec<Mission>);

impl Missions {
    pub fn get_mission_by_id(&self, id: u16) -> Option<Mission> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == id) {
            return Some(mission.clone());
        }
        None
    }

    pub fn get_mission_by_name(&self, name: &str) -> Option<Mission> {
        return self.0.iter().find(|mission| mission.name == name).cloned();
    }

    pub fn assign_recruit_id_to_mission(&mut self, mission_id: u16, recruit_id: Uuid) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.assign_recruit_by_id(recruit_id);
        }
    }

    pub fn decrement_days_left_by_mission_id(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.decrement_days_left();
        }
    }

    pub fn attribute_days_left_to_mission(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.attribute_days_left();
        }
    }

    pub fn is_mission_over(&self, mission_id: u16) -> bool {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.days_left.is_none();
        }
        false
    }

    pub fn get_recruit_send_id_by_mission_id(&self, mission_id: u16) -> Option<Uuid> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.recruit_send;
        }
        None
    }

    pub fn desassign_recruit_to_mission(&mut self, mission_id: u16) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.desassign_recruit();
        }
    }

    pub fn get_mission_enemmy_level_by_id(&self, mission_id: u16) -> Option<u8> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return Some(mission.ennemy.level);
        }
        None
    }

    pub fn get_percent_of_victory_by_mission_id(&self, mission_id: u16) -> Option<u32> {
        if let Some(mission) = self.0.iter().find(|mission| mission.id == mission_id) {
            return mission.percent_of_victory;
        }
        None
    }

    pub fn attribute_percent_of_victory_to_mission(
        &mut self,
        mission_id: u16,
        percent_of_victory: u32,
    ) {
        if let Some(mission) = self.0.iter_mut().find(|mission| mission.id == mission_id) {
            mission.percent_of_victory = Some(percent_of_victory);
        }
    }

    pub fn get_missions_by_ids(&self, ids: Vec<u16>) -> Vec<Mission> {
        let mut missions = vec![];

        for id in ids {
            if let Some(mission) = self.get_mission_by_id(id) {
                missions.push(mission);
            }
        }

        missions
    }
}

#[derive(Debug, Component, Clone, Eq, PartialEq, Hash)]
pub struct Mission {
    pub days_left: Option<u8>,
    pub days: u8,
    pub ennemy: Ennemy,
    pub id: u16,
    pub level: u8,
    pub name: String,
    pub percent_of_victory: Option<u32>,
    pub recruit_send: Option<Uuid>,
    pub unlocked: bool,
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

impl Default for Missions {
    fn default() -> Self {
        Self(vec![
            Mission {
                days_left: None,
                days: 1,
                id: 1,
                level: 1,
                name: "Mission 1".to_string(),
                percent_of_victory: None,
                recruit_send: None,
                ennemy: Ennemy {
                    image_atlas_index: 5,
                    endurance: 10,
                    experience: 0,
                    intelligence: 5,
                    level: 1,
                    name: "Ennemy 1".to_string(),
                    strength: 10,
                },
                unlocked: true,
            },
            Mission {
                days_left: None,
                days: 1,
                id: 2,
                level: 2,
                name: "Mission 2".to_string(),
                percent_of_victory: None,
                recruit_send: None,
                ennemy: Ennemy {
                    image_atlas_index: 4,
                    endurance: 15,
                    experience: 0,
                    intelligence: 7,
                    level: 2,
                    name: "Ennemy 2".to_string(),
                    strength: 15,
                },
                unlocked: true,
            },
            Mission {
                days_left: Some(1),
                days: 2,
                id: 3,
                level: 3,
                name: "Mission 3".to_string(),
                percent_of_victory: None,
                recruit_send: None,
                ennemy: Ennemy {
                    image_atlas_index: 3,
                    endurance: 20,
                    experience: 0,
                    intelligence: 10,
                    level: 3,
                    name: "Ennemy 3".to_string(),
                    strength: 20,
                },
                unlocked: false,
            },
        ])
    }
}
