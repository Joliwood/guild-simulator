use crate::structs::general_structs::PlayerStats;
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

// WIP
// Struct to represent a Toast Message
#[derive(Component)]
struct Toast;

pub fn notification_toast(
    _asset_server: Res<AssetServer>,
    mut commands: Commands,
    _player_stats: Res<PlayerStats>,
) {
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(60.),
                height: Val::Px(60.),
                border: UiRect::all(Val::Px(3.)),
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            // image: asset_server.load(item_atlas_path).clone().into(),
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::all(Val::Px(10.)),
            ..default()
        },
        // TextureAtlas {
        //     index: item_image_atlas_index.into(),
        //     layout: texture_atlas_layouts.add(layout),
        // },
        Tooltip::cursor("hello there").with_activation(TooltipActivation::IMMEDIATE),
    ));
}
