// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod audio;
mod systems;
mod ui;

// ! Make crash the UI
// use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            // EditorPlugin::default(),
        ))
        // Background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, (
            audio::audio_source::audio_source,
            ui::ui_setup::ui_setup,
            setup,
        ))
        .add_systems(Update, (
            ui::menu_button::menu_button,
            bevy::window::close_on_esc, 
            systems::inputs::mouse_click_system::mouse_click_system,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
