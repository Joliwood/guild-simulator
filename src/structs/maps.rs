use bevy::prelude::*;
use uuid::Uuid;

use super::missions::{Mission, Missions};

// #[derive(Debug, Component, Resource)]
// pub struct Objective {
//     pub id: Uuid,
//     pub name: String,
//     pub description: String,
//     pub is_completed: bool,
// }

// impl Objective {
//     pub fn is_completed(&self) -> bool {
//         // WIP - Pour que l'objective se remplisse automatiquement ?
//         self.is_completed
//     }
// }

// #[derive(Debug, Component, Resource)]
// pub struct Objectives(pub Vec<Objective>);

#[derive(Debug, Component, Resource)]
pub struct Maps(pub Vec<Map>);

#[derive(Debug, Component, Resource)]
pub struct Map {
    pub description: String,
    pub id: Uuid,
    pub map_mission_ids: Vec<u16>,
    pub name: String,
    // pub image_atlas_index: u16,
}

#[derive(Debug, Component, Resource)]
pub struct SelectedMap {
    pub map_id: Option<Uuid>,
}

// pub fn get_mission_by_name(&self, name: &str) -> Option<Mission> {
//   return self.0.iter().find(|mission| mission.name == name).cloned();
// }

impl Default for Maps {
    fn default() -> Self {
        Self(vec![
            Map {
                description: "Tutorial description".to_string(),
                id: Uuid::new_v4(),
                map_mission_ids: vec![1, 2, 3],
                name: "Campagn tuto".to_string(),
            },
            Map {
                description: "Campaign 1 description".to_string(),
                id: Uuid::new_v4(),
                map_mission_ids: vec![],
                name: "Campaign 2".to_string(),
            },
            Map {
                description: "Campaign 2 description".to_string(),
                id: Uuid::new_v4(),
                map_mission_ids: vec![],
                name: "Campaign 2".to_string(),
            },
        ])
    }
}
