use crate::enums::MapImageEnum;
use bevy::prelude::*;

#[derive(Debug, Component, Resource, Clone)]
pub struct Maps(pub Vec<Map>);

impl Maps {
    pub fn get_map_by_optional_id(&self, id: Option<u16>) -> Option<Map> {
        if let Some(id) = id {
            return self.0.iter().find(|map| map.id == id).cloned();
        }
        None
    }

    pub fn get_uuid_of_tuto_map(&self) -> Option<u16> {
        self.0
            .iter()
            .find(|map| map.name == "Campagn tuto")
            .map(|map| map.id)
    }
}

#[derive(Debug, Component, Resource, Clone)]
pub struct Map {
    pub description: String,
    pub id: u16,
    pub map_mission_ids: Vec<u16>,
    pub name: String,
    pub unlocked: bool,
    // pub image_atlas_index: u16,
    pub image: MapImageEnum,
}

// let first_map_uuid: Uuid = Uuid::new_v4();

#[derive(Debug, Component, Resource, Clone)]
pub struct SelectedMapId(pub Option<u16>);

impl Default for SelectedMapId {
    fn default() -> Self {
        Self(Some(1))
    }
}

impl Default for Maps {
    fn default() -> Self {
        Self(vec![
            Map {
                description: "Map gived by the mayor, marking vagrant camps causing trouble. Taking out their leader could make the town safer.".to_string(),
                id: 1,
                map_mission_ids: vec![1, 2, 3, 4, 5, 6],
                name: "Troublemaker's Territory".to_string(),
                unlocked: true,
                // image_atlas_index: 0,
                image: MapImageEnum::CampagnTuto,
            },
            Map {
                description: "Campaign 1 description".to_string(),
                id: 2,
                map_mission_ids: vec![],
                name: "Campaign 2".to_string(),
                unlocked: true,
                // image_atlas_index: 1,
                image: MapImageEnum::CampagnTuto,
            },
            Map {
                description: "Campaign 2 description".to_string(),
                id: 3,
                map_mission_ids: vec![],
                name: "Campaign 2".to_string(),
                unlocked: false,
                // image_atlas_index: 2,
                image: MapImageEnum::CampagnTuto,
            },
        ])
    }
}
