use crate::structs::UniqueId;
use crate::styles::CustomButton;
use crate::ui::interface::gold_counter::MyAssets;
use crate::ui::ui_constants::WOOD_COLOR;
use bevy::prelude::*;

pub fn gold_button(
    asset_server: &Res<AssetServer>,
    commands: &mut ChildBuilder,
    image_assets: Res<MyAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // the sprite sheet has 16 sprites arranged in a row, and they are all 500px x 500px
    let texture_handle = asset_server.load("images/ui/buttons_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(500), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands
        // .spawn(CustomButton::GoldButton.bundle(&asset_server, image_assets.clone()))
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    width: Val::Px(60.0),
                    height: Val::Px(60.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: texture_handle.clone().into(),
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            TextureAtlas {
                index: 11,
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
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
