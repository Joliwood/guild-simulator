use crate::{my_assets::FONT_FIRA, structs::recruits::RecruitStats};
use bevy::prelude::*;

pub fn recruit_sent_picture(
    commands: &mut ChildBuilder,
    recruit_sent: &RecruitStats,
    my_assets: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(200, 400),
        7,
        1,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let recruit_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(recruit_layout);

    // Recruit Send Block
    commands
        .spawn(Node {
            display: Display::Flex,
            width: Val::Px(100.),
            height: Val::Px(200.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|recruit_block| {
            // Text: "Recruit sent"
            recruit_block.spawn((
                Text::new("Recruit sent"),
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            recruit_block.spawn((
                UiImage::from_atlas_image(
                    my_assets.load("images/recruits/recruit_picture_atlas.png"),
                    TextureAtlas {
                        index: recruit_sent.image_atlas_index.into(),
                        layout: recruit_texture_atlas_layout.clone(),
                    },
                ),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                GlobalZIndex(1),
            ));
        });
}
