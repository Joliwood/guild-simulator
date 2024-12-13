use crate::structs::trigger_structs::ResetRoomTrigger;
use bevy::prelude::*;

pub fn room_store(my_assets: &Res<AssetServer>, commands: &mut Commands) {
    // let imager_handler: Handle<Image> = my_assets.load("images/store.png");

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .insert(Name::new("Store room"))
        .insert(ResetRoomTrigger)
        // Image background node
        .with_children(|ui_container: &mut ChildBuilder| {
            ui_container.spawn((
                UiImage {
                    image: my_assets.load("images/rooms/store/store.png"),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                GlobalZIndex(-1),
            ));
        });
}
