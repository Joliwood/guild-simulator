use crate::{my_assets::MyAssets, structs::recruits::RecruitStats};
use bevy::prelude::*;

pub fn recruit_sent_picture(
    commands: &mut ChildBuilder,
    recruit_sent: &RecruitStats,
    my_assets: &Res<MyAssets>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let recruit_layout = TextureAtlasLayout::from_grid(
        UVec2::new(1400, 400),
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
                    font: my_assets.fira_sans_bold.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            recruit_block.spawn((
                UiImage::from_atlas_image(
                    my_assets.recruit_picture_atlas.clone().into(),
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
                ZIndex(1),
            ));
        });
}
