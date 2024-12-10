use crate::{
    enums::RoomEnum,
    structs::{
        daily_events_folder::daily_events::DailyEvents,
        general_structs::MissionReportsModalVisible,
        maps::{Maps, SelectedMapId},
        missions::{MissionReports, Missions},
        player_stats::PlayerStats,
        recruits::SelectedRecruitForEquipment,
        trigger_structs::ResetRoomTrigger,
    },
    ui::rooms::{
        barrack::barrack_room::spawn_room_barrack,
        command_room::room_command_room::room_command_room, office::room_office::room_office,
        room_store::room_store,
    },
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn update_room(
    my_assets: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
    mut commands: Commands,
    query: Query<Entity, With<ResetRoomTrigger>>,
    mut previous_room: Local<Option<RoomEnum>>, // Local state for previous room
    selected_recruit_for_equipment: Res<SelectedRecruitForEquipment>,
    missions: Res<Missions>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mission_reports: Res<MissionReports>,
    mission_reports_modal_visibility: ResMut<MissionReportsModalVisible>,
    maps: Res<Maps>,
    selected_map_id: Res<SelectedMapId>,
    daily_events: Res<DailyEvents>,
) {
    room_office(
        &my_assets,
        &mut commands,
        &mission_reports,
        mission_reports_modal_visibility,
        &daily_events,
        &mut texture_atlas_layouts,
    );

    spawn_room_barrack(
        &my_assets,
        &mut commands,
        &player_stats,
        &selected_recruit_for_equipment,
        &mut texture_atlas_layouts,
    );

    room_command_room(
        &my_assets,
        &mut commands,
        missions,
        selected_map_id,
        maps,
        &player_stats,
        &mut texture_atlas_layouts,
    );

    // if player_stats.is_changed() {
    //     // Compare the current room with the previously stored value
    //     if previous_room
    //         .as_ref()
    //         .map_or(true, |prev| *prev != player_stats.room)
    //     {
    //         // Update local state
    //         *previous_room = Some(player_stats.clone().room);

    //         // Despawn existing entities marked with `ResetRoomTrigger`
    //         for entity in query.iter() {
    //             commands.entity(entity).despawn_recursive();
    //         }

    //         // Spawn new room based on `player_stats.room`
    //         match player_stats.room {
    //         RoomEnum::Office => room_office(
    //             &my_assets,
    //             &mut commands,
    //             &mission_reports,
    //             mission_reports_modal_visibility,
    //             &daily_events,
    //             &mut texture_atlas_layouts,
    //         ),
    //         RoomEnum::Barrack => spawn_room_barrack(
    //             &my_assets,
    //             &mut commands,
    //             &player_stats,
    //             &selected_recruit_for_equipment,
    //             &mut texture_atlas_layouts,
    //         ),
    //         RoomEnum::Store => room_store(&my_assets, &mut commands),
    //         RoomEnum::CommandRoom => room_command_room(
    //             &my_assets,
    //             &mut commands,
    //             missions,
    //             selected_map_id,
    //             maps,
    //             &player_stats,
    //             &mut texture_atlas_layouts,
    //         ),
    //         }
    //     }
    // }
}
