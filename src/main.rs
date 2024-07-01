// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod audio;
mod enums;
mod structs;
mod systems;
mod ui;

// ! Make crash the UI
use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use structs::PlayerStats;

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
                systems::inputs::mouse_systems::mouse_init,
            ),
        )
        .add_systems(
            Update,
            (
                ui::menu_button::menu_button,
                // systems::experimental::close_on_esc::close_on_esc,
                systems::inputs::mouse_systems::mouse_click_system,
                systems::updates::update_gold_counter::update_gold_counter,
            ),
        )
        .run()
}
