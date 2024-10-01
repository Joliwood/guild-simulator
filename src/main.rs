// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod audio;
mod custom_components;
mod enums;
mod structs;
mod systems;
mod ui;
mod utils;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyri_tooltip::prelude::*;
use structs::general_structs::{
    MissionModalVisible, MissionNotificationsNumber, Missions, PlayerStats, SelectedMission,
    SelectedRecruit, UniqueId,
};
use ui::{
    interface::gold_counter::MyAssets,
    rooms::barrack::recruits_list::weapon_button::update_weapon_button,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct MySystems;

#[derive(Component)]
pub struct AlertButton;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Desactivate on testing
            // WorldInspectorPlugin::new(),
            TooltipPlugin::default(),
            // AlertsPlugin::new(),
        ))
        .insert_resource(PlayerStats::default())
        .insert_resource(Missions::default())
        .insert_resource(SelectedRecruit::default())
        .insert_resource(SelectedMission::default())
        .insert_resource(MissionModalVisible(false))
        .insert_resource(MissionNotificationsNumber(0))
        .init_collection::<MyAssets>()
        .add_systems(
            Startup,
            (
                audio::audio_source::audio_source,
                systems::camera::camera_setup::camera_setup,
                systems::inputs::mouse_systems::mouse_init,
                ui::buttons::room_arrows::room_bottom_arrow_button,
                ui::buttons::room_arrows::room_left_arrow_button,
                ui::buttons::room_arrows::room_right_arrow_button,
                ui::buttons::room_arrows::room_top_arrow_button,
                ui::interface::gold_counter::gold_counter,
                ui::interface::room_interface_text::room_interface_text,
                ui::rooms::room_setup::room_setup,
                systems::recruits::hiring_setup::hiring_setup,
            ),
        )
        .add_systems(
            Update,
            (
                systems::updates::update_buttons::move_room_from_keyboard,
                systems::inputs::mouse_systems::mouse_click_system,
                systems::updates::update_gold_counter::update_gold_counter,
                systems::updates::update_room_interface_text::update_room_interface_text,
                systems::updates::update_room::update_room,
                systems::updates::update_buttons::mouse_interaction_updates,
                systems::updates::update_buttons::buttons_disable_updates,
                systems::updates::update_buttons::select_recruit_button,
                systems::updates::update_buttons::select_mission_button,
                systems::updates::update_buttons::assign_recruit_to_mission,
                systems::updates::update_buttons::close_mission_modal,
                systems::updates::update_buttons::start_mission_button,
                systems::updates::update_buttons::select_item_in_inventory,
                systems::updates::update_recruit_infos::update_recruit_infos,
                systems::updates::update_selected_recruit::update_selected_mission_recruit_id,
                systems::updates::update_selected_recruit::update_update_selected_mission_percentage_of_victory,
                systems::updates::delete_notifications_on_click::delete_notifications_on_click,
                ui::interface::notifications::spawn_or_update_notification::spawn_or_update_notification,
                ui::modals::mission_details_modal::display_mission_modal,
                // update_weapon_ui_on_player_stats_change,
            ),
        )
        .run()
}

// pub fn update_weapon_ui_on_player_stats_change(
//     mut commands: Commands,
//     player_stats: Res<PlayerStats>,
//     selected_recruit: Res<SelectedRecruit>,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//     mut weapon_button_query: Query<(Entity, &UniqueId), With<Button>>,
//     parent_query: Query<Entity, With<Parent>>, // The container holding the buttons
// ) {
//     // Only proceed if the PlayerStats resource has changed.
//     if !player_stats.is_changed() {
//         return;
//     }

//     // Find the selected recruit and, if it exists, update the weapon button.
//     if let Some(selected_recruit_stats) = &selected_recruit.0 {
//         let recruit_id = selected_recruit_stats.id.to_string();

//         // Loop through all weapon buttons to find the one belonging to the selected recruit.
//         for (entity, unique_id) in &mut weapon_button_query {
//             if unique_id.0.starts_with("item_in_inventory_") {
//                 if let Some(current_recruit_id) = unique_id.0.strip_prefix("item_in_inventory_") {
//                     if current_recruit_id == recruit_id {
//                         // Despawn the current button so we can recreate it.
//                         commands.entity(entity).despawn_recursive();

//                         // Get the parent entity to add a new button.
//                         if let Ok(parent_entity) = parent_query.get(entity) {
//                             // Add the updated button for the selected recruit's weapon.
//                             commands.entity(parent_entity).with_children(|parent| {
//                                 update_weapon_button(
//                                     parent,
//                                     &asset_server,
//                                     selected_recruit_stats,
//                                     &mut texture_atlas_layouts,
//                                 );
//                             });
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
