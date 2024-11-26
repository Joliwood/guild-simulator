use crate::{
    enums::{ColorPaletteEnum, TextureAtlasLayoutEnum},
    my_assets::FONT_FIRA,
    structs::{
        player_stats::PlayerStats,
        trigger_structs::{PlayerDayTrigger, RealTimeDayProgressBarTrigger, SleepButtonTrigger},
    },
    utils::get_layout,
};
use bevy::prelude::*;

pub fn sleep_button(
    commands: &mut Commands,
    my_assets: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    // day_time: &Res<DayTime>,
) {
    let sleep_button_layout = get_layout(TextureAtlasLayoutEnum::SleepButton);
    let sleep_button_atlas_layout = texture_atlas_layouts.add(sleep_button_layout);
    // let text_hello = t!("hello");

    commands
        .spawn((
            UiImage {
                image: my_assets.load("images/hud/sleep_button_container2.png"),
                ..default()
            },
            Node {
                width: Val::Px(143.),
                height: Val::Px(70.),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                ..default()
            },
            GlobalZIndex(3),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    UiImage::from_atlas_image(
                        my_assets.load("images/hud/sleep_button_atlas.png"),
                        TextureAtlas {
                            index: 1,
                            layout: sleep_button_atlas_layout.clone(),
                        },
                    )
                    .with_mode(NodeImageMode::Stretch),
                    Node {
                        width: Val::Px(70.),
                        height: Val::Px(70.),
                        border: UiRect::all(Val::Px(3.)),
                        ..default()
                    },
                    GlobalZIndex(2),
                    BorderColor(Color::NONE),
                    BorderRadius::MAX,
                    SleepButtonTrigger,
                ))
                .insert(Name::new("Sleep Button Icon"));

            parent.spawn((
                Text::new(format!("{} : {}", t!("day"), player_stats.day)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(7.),
                    left: Val::Px(80.),
                    ..default()
                },
                TextFont {
                    font: my_assets.load(FONT_FIRA),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                PlayerDayTrigger,
            ));
            // .insert(PlayerDayTrigger);

            // Progress bar
            parent.spawn((
                UiImage::solid_color(ColorPaletteEnum::Success.as_color()),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.),
                    left: Val::Px(60.),
                    width: Val::Px(0.0),
                    height: Val::Px(3.0),
                    margin: UiRect {
                        bottom: Val::Px(3.),
                        ..default()
                    },
                    ..default()
                },
                BorderRadius::MAX,
                GlobalZIndex(5),
                RealTimeDayProgressBarTrigger,
            ));
        });
}
