use crate::structs::general_structs::PlayerStats;
use bevy::prelude::*;

pub fn spawn_right_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
) {
    // Container for the inventory
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            // Check if the inventory is available (Some)
            if let Some(inventory) = &player_stats.inventory {
                // Iterate over the player's inventory and display each item
                for item in inventory.iter() {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("{} x{}", item.name, item.quantity),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                }
            } else {
                // Display a message if the inventory is None
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "No items in inventory",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });
            }
        });
}
