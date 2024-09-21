use crate::structs::{equipments::Item, general_structs::PlayerStats};
use bevy::prelude::*;

pub fn spawn_right_container(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/equipments/weapons_atlas.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(2900, 400),
        6,
        1,
        Some(UVec2::new(000, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Container for the inventory
    parent
        .spawn(NodeBundle {
            style: Style {
                // flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
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
            if &player_stats.inventory.len() > &0 {
                // Iterate over the player's inventory and display each item
                for item in player_stats.inventory.iter() {
                    let image_atlas_index = match item {
                        Item::Weapon(weapon) => weapon.image_atlas_index,
                        Item::Armor(armor) => armor.image_atlas_index,
                        Item::Scroll(scroll, _) => scroll.image_atlas_index,
                    };

                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(60.),
                                height: Val::Px(60.),
                                border: UiRect::all(Val::Px(3.)),
                                ..default()
                            },
                            image: texture_handle.clone().into(),
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius::all(Val::Px(10.)),
                            ..default()
                        },
                        TextureAtlas {
                            index: image_atlas_index.into(),
                            layout: texture_atlas_layout.clone(),
                        },
                    ));
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
