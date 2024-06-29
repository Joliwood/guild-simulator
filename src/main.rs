// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod audio;
mod systems;
mod ui;

// ! Make crash the UI
use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct GoldCountText;

#[derive(Resource)]
struct PlayerStats {
    golds: i32,
    // troups_count: i32,
}

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
            // troups_count: 0,
        }
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // WorldInspectorPlugin::new(),
            // EditorPlugin::default(),
        ))
        // Background color
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .insert_resource(PlayerStats::default())
        .add_systems(
            Startup,
            (
                audio::audio_source::audio_source,
                systems::camera::camera_setup::camera_setup,
                ui::gold_counter::gold_counter,
                ui::ui_setup::ui_setup,
            ),
        )
        .add_systems(
            Update,
            (
                ui::menu_button::menu_button,
                // systems::experimental::close_on_esc::close_on_esc,
                systems::inputs::mouse_click_system::mouse_click_system,
                systems::updates::update_gold_counter::update_gold_counter,
            ),
        )
        .run()
}
