use crate::{systems::constants::*, GoldCountText};
use bevy::prelude::*;

/// All the UI logic and components will be setup here
///
/// # Parameters
/// - `commands`: Bevy's commands
/// - `asset_server`: Bevy's asset server (at this moment, only fonts)
pub fn ui_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let image_handle = asset_server.load("images/desktop.png");
    let wood_color = Color::srgba(193.0 / 255.0, 154.0 / 255.0, 107.0 / 255.0, 1.0);

    // At this moment, Bevy render doesn't support camelCase img
    println!("Loaded image handle: {:?}", image_handle);

    commands
        // Global UI container (100% screen)
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            // Temporary background color
            background_color: wood_color.into(),
            ..default()
        })
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container
                .spawn(ImageBundle {
                    image: image_handle.clone().into(),
                    style: Style {
                        // The position absolute make the gold counter visible (z-index)
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(90.0),
                        margin: UiRect {
                            left: Val::Percent(5.0),
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                        },
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(GoldCountText);
        })
        .with_children(|settings_button: &mut ChildBuilder| {
            settings_button
                .spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    image: UiImage::default().with_color(NORMAL_BUTTON),
                    ..default()
                })
                .with_children(|settings_button| {
                    settings_button.spawn(TextBundle::from_section(
                        "X",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
