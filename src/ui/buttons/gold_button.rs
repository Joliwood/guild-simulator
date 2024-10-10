use crate::{structs::general_structs::UniqueId, ui::interface::gold_counter::MyAssets};
use bevy::prelude::*;

pub fn gold_button(
    asset_server: &Res<AssetServer>,
    commands: &mut ChildBuilder,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    my_assets: Res<MyAssets>,
) {
    // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px
    // let texture_handle = asset_server.load("images/ui/buttons_atlas.png");
    // let texture_handle = my_assets.test_button.clone();
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(5436, 3809),
        5,
        6,
        Some(UVec2::new(0, 0)),
        Some(UVec2::new(0, 0)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    width: Val::Px(60.),
                    height: Val::Px(60.),
                    border: UiRect::all(Val::Px(3.)),
                    ..default()
                },
                // image: texture_handle.clone().into(),
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(10.)),
                ..default()
            },
            TextureAtlas {
                index: 25,
                layout: texture_atlas_layout.clone(),
            },
        ))
        .insert(Name::new("Gold button"))
        .insert(UniqueId("menu_button_id".to_string()))
        // Text inside the button
        .with_children(|settings_button| {
            settings_button.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
