#![allow(dead_code)]
use crate::{
    my_assets::MyAssets,
    structs::{
        general_structs::UniqueId, player_stats::PlayerStats, trigger_structs::GoldCountTrigger,
    },
    systems::systems_constants::NORMAL_BUTTON,
    ui::{styles::containers_styles::basic_button_style, ui_constants::WOOD_COLOR},
};
use bevy::prelude::*;

pub fn gold_counter(
    my_assets: Res<MyAssets>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
    // image_assets: Res<MyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(500), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // commands
    //     .spawn(NodeBundle {
    //         // style: basic_button_style(),
    //         // background_color: BackgroundColor(WOOD_COLOR),
    //         ..default()
    //     })
    //     .insert(Name::new("Gold counter"));
    // // Gold icon
    // .with_children(|ui_container: &mut ChildBuilder| {
    //     ui_container.spawn(ImageBundle {
    //         image: my_assets.gold.clone().into(),
    //         style: Style {
    //             // The position absolute make the gold counter visible (z-index)
    //             width: Val::Px(24.0),
    //             height: Val::Px(24.0),
    //             align_self: AlignSelf::Center,
    //             justify_self: JustifySelf::Center,
    //             ..default()
    //         },
    //         ..default()
    //     });
    // })
    // // ! WIP - The only sprite bundle that works
    // .with_children(|ui_container: &mut ChildBuilder| {
    //     ui_container.spawn((
    //         SpriteBundle {
    //             texture: my_assets.buttons_atlas.clone(),
    //             transform: Transform {
    //                 translation: Vec3::new(100.0, 0.0, 0.0),
    //                 scale: Vec3::splat(0.2),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         TextureAtlas {
    //             layout: texture_atlas_layout.clone(),
    //             index: 15,
    //         },
    //     ));
    // })
    // .insert(Name::new("Gold icon"))
    // // Gold counter text
    // .with_children(|ui_container: &mut ChildBuilder| {
    //     ui_container
    //         .spawn(TextBundle {
    //             text: Text::from_section(
    //                 format! {"Guild level : {} | {gold_counter}", player_stats.guild_level, gold_counter = player_stats.golds},
    //                 TextStyle {
    //                     font: my_assets.fira_sans_bold.clone(),
    //                     font_size: 40.0,
    //                     color: Color::BLACK,
    //                 },
    //             ),
    //             ..default()
    //         })
    //         .insert(GoldCountTrigger)
    //         .insert(Name::new("Gold counter text"));
    // })
    // .with_children(|ui_container: &mut ChildBuilder| {
    //     ui_container
    //         .spawn(ButtonBundle {
    //             style: Style {
    //                 display: Display::Flex,
    //                 justify_content: JustifyContent::Center,
    //                 width: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage::default().with_color(NORMAL_BUTTON),
    //             ..default()
    //         })
    //         .insert(Name::new("Hire recruit button"))
    //         .insert(UniqueId("waz".to_string()))
    //         .with_children(|ui_container: &mut ChildBuilder| {
    //             ui_container.spawn(TextBundle::from_section(
    //                 "Buy",
    //                 TextStyle {
    //                     font: my_assets.fira_sans_bold.clone(),
    //                     font_size: 20.0,
    //                     color: Color::BLACK,
    //                 },
    //             ));
    //         });
    // });
}
