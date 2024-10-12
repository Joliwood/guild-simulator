use crate::structs::recruits::RecruitStats;
use bevy::prelude::*;

pub fn recruit_sent_picture(
    commands: &mut ChildBuilder,
    recruit_sent: &RecruitStats,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_image_handle: Handle<Image> =
        asset_server.load("images/recruits/recruit_picture_atlas.png");

    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(800, 200),
        5,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    // Recruit Send Block
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
        .with_children(|recruit_block| {
            // Text: "Recruit sent"
            recruit_block.spawn(TextBundle {
                text: Text::from_section(
                    "Recruit sent",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });

            recruit_block.spawn((
                ImageBundle {
                    image: recruit_image_handle.into(),
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
                    index: recruit_sent.image_atlas_index.into(),
                    layout: recruit_texture_atlas_layout.clone(),
                },
            ));
        });
}
