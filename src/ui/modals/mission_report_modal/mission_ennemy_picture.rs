use crate::structs::general_structs::Ennemy;
use bevy::prelude::*;

pub fn mission_ennemy_picture(
    commands: &mut ChildBuilder,
    ennemy: &Ennemy,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let ennemy_image_handle: Handle<Image> =
        asset_server.load("images/missions/ennemy_picture_atlas.png");

    let ennemy_layout = TextureAtlasLayout::from_grid(
        UVec2::new(1200, 200),
        6,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let ennemy_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(ennemy_layout);

    // Ennemy Block
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Px(100.),
                height: Val::Px(200.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|ennemy_block| {
            // Text: "ennemy sent"
            ennemy_block.spawn(TextBundle {
                text: Text::from_section(
                    "ennemy sent",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });

            ennemy_block.spawn((
                ImageBundle {
                    image: ennemy_image_handle.into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    z_index: ZIndex::Global(1),
                    ..default()
                },
                TextureAtlas {
                    index: ennemy.image_atlas_index.into(),
                    layout: ennemy_texture_atlas_layout.clone(),
                },
            ));
        });
}
