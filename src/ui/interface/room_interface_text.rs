use crate::{
    structs::{PlayerStats, PlayerStatsRoomTrigger},
    ui::ui_constants::WOOD_COLOR,
};
use bevy::prelude::*;

pub fn room_interface_text(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
) {
    commands
        // Container
        .spawn(NodeBundle {
            style: Style {
                // The position absolute make the gold counter visible (z-index)
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                display: Display::Flex,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                width: Val::Auto,
                height: Val::Px(36.0),
                ..default()
            },
            background_color: BackgroundColor(WOOD_COLOR),
            ..default()
        })
        .insert(Name::new("Room interface text"))
        // Room text
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(TextBundle {
                    text: Text::from_section(
                        format! {"{:?}", player_stats.room},
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(PlayerStatsRoomTrigger);
        });
}
