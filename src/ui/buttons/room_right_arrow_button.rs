use bevy::prelude::*;

use crate::{
    structs::UniqueId, systems::systems_constants::NORMAL_BUTTON, ui::ui_constants::WOOD_COLOR,
};

pub fn room_right_arrow_button(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    // player_stats: Res<PlayerStats>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // The position absolute make the gold counter visible (z-index)
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                display: Display::Flex,
                margin: UiRect::all(Val::Auto),
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
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(ButtonBundle {
                    style: Style { ..default() },
                    border_color: BorderColor(Color::BLACK),
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                .insert(UniqueId("room_right_arrow_id".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        ">",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
