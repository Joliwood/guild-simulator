use crate::structs::general_structs::PlayerStats;
use bevy::prelude::*;
use pyri_tooltip::{Tooltip, TooltipActivation};

pub fn notification_toast(
    _player_stats: Res<PlayerStats>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("images/ui/notification_atlas.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(200, 50),
        4,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(50.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                width: Val::Px(60.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(60.),
                        height: Val::Px(60.),
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    image: texture_handle.clone().into(),
                    border_radius: BorderRadius::all(Val::Px(10.)),
                    ..default()
                },
                TextureAtlas {
                    index: 0,
                    layout: texture_atlas_layout.clone(),
                },
                Tooltip::cursor("Mission finished !\n\n Go check out your rapports")
                    .with_activation(TooltipActivation::IMMEDIATE),
            ));
        });
}
