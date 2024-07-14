use crate::{
    structs::{GoldCountTrigger, PlayerStats, UniqueId},
    systems::systems_constants::NORMAL_BUTTON,
    ui::ui_constants::WOOD_COLOR,
};
use bevy::prelude::*;

pub fn gold_counter(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    player_stats: Res<PlayerStats>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                width: Val::Auto,
                height: Val::Px(36.0),
                ..default()
            },
            background_color: BackgroundColor(WOOD_COLOR),
            ..default()
        })
        // Gold icon
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn(ImageBundle {
                image: asset_server.load("images/gold.png").into(),
                style: Style {
                    // The position absolute make the gold counter visible (z-index)
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            });
        })
        // Gold counter text
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(TextBundle {
                    text: Text::from_section(
                        format! {"{gold_counter}", gold_counter = player_stats.golds},
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(GoldCountTrigger);
        })
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    // border_color: BorderColor(Color::BLACK),
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                .insert(UniqueId("waz".to_string()))
                .with_children(|ui_container: &mut ChildBuilder| {
                    ui_container.spawn(TextBundle::from_section(
                        "Buy",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}
