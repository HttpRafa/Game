use bevy::app::App;
use bevy::input::common_conditions::input_toggle_active;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::utils::HashSet;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::client::state::ingame::local_player::LocalPlayer;
use crate::client::state::GameState;
use crate::client::world::{spawn_chunk, world_to_chunk_position, Chunk};
use crate::registry::atlas::GameTextures;
use crate::registry::chunk_data::{CHUNK_LOAD_SIZE, CHUNK_SIZE, RENDER_CHUNK_SIZE, TILE_SIZE};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .insert_resource(ChunkManager::default())
            .add_systems(OnExit(GameState::InGame), cleanup_chunks)
            .add_systems(
                Update,
                spawn_chunks_around_player.run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, despawn_chunks.run_if(in_state(GameState::InGame)))
            .add_plugins(
                ResourceInspectorPlugin::<ChunkManager>::default()
                    .run_if(input_toggle_active(false, KeyCode::Numpad2)),
            );
    }
}

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

fn spawn_chunks_around_player(
    mut commands: Commands,
    textures: Res<GameTextures>,
    player_transform: Query<&Transform, With<LocalPlayer>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let player_transform = player_transform.single();
    let chunk_position = world_to_chunk_position(&player_transform.translation.xy());
    for x in
        (chunk_position.x - CHUNK_LOAD_SIZE.x as i32)..(chunk_position.x + CHUNK_LOAD_SIZE.x as i32)
    {
        for y in (chunk_position.y - CHUNK_LOAD_SIZE.y as i32)
            ..(chunk_position.y + CHUNK_LOAD_SIZE.y as i32)
        {
            if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                spawn_chunk(IVec2::new(x, y), Chunk::default(), &mut commands, &textures);
            }
        }
    }
}

fn despawn_chunks(
    mut commands: Commands,
    player_transform: Query<&Transform, With<LocalPlayer>>,
    chunks: Query<(Entity, &Transform), With<Chunk>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let player_transform = player_transform.single();
    for (entity, chunk_transform) in chunks.iter() {
        let chunk_position = chunk_transform.translation.xy();
        let distance = player_transform.translation.xy().distance(chunk_position);
        if distance > (TILE_SIZE.x * RENDER_CHUNK_SIZE.x as f32) * 1.75 {
            let x = (chunk_position.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
            let y = (chunk_position.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
            chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn cleanup_chunks(
    mut commands: Commands,
    chunks: Query<Entity, With<Chunk>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for chunk in &chunks {
        commands.entity(chunk).despawn_recursive();
    }
    chunk_manager.spawned_chunks.clear();
}
