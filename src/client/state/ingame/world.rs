use bevy::app::App;
use bevy::core::Name;
use bevy::math::Vec3Swizzles;
use bevy::prelude::{BuildChildren, Commands, Component, default, DespawnRecursiveExt, Entity, in_state, IntoSystemConfigs, IVec2, OnExit, Plugin, Query, Res, ResMut, Resource, Transform, Update, Vec2, Vec3, With};
use bevy::utils::HashSet;
use bevy_ecs_tilemap::{TilemapBundle, TilemapPlugin};
use bevy_ecs_tilemap::prelude::{TileBundle, TilemapId, TilemapRenderSettings, TilemapTexture, TilePos, TileStorage, TileTextureIndex};
use rand::{Rng, thread_rng};
use crate::client::state::GameState;
use crate::client::state::ingame::local_player::LocalPlayer;
use crate::client::texture::{GameTextures, WORLD_GROUND_TILES};

use crate::client::y_sorting::YSort;
use crate::common::{CHUNK_LOAD_SIZE, CHUNK_SIZE, RENDER_CHUNK_SIZE, TILE_SIZE};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..default()
        }).add_plugins(TilemapPlugin)
            .insert_resource(ChunkManager::default())
            .add_systems(OnExit(GameState::InGame), cleanup_chunks)
            .add_systems(Update, spawn_chunks_around_player.run_if(in_state(GameState::InGame)))
            .add_systems(Update, despawn_chunks.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Resource, Default)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Component)]
struct Chunk;

fn spawn_chunk(commands: &mut Commands, textures: &GameTextures, chunk_pos: IVec2) {
    let tilemap_entity = commands.spawn((Chunk, Name::new(format!("Chunk {} {}", chunk_pos.x, chunk_pos.y)))).id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.x {
            let tile_pos = TilePos { x, y};
            let tile_entity = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(thread_rng().gen_range(0..WORLD_GROUND_TILES.columns as u32)),
                ..default()
            }).id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));
    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(textures.world_ground_tiles.clone()),
        tile_size: TILE_SIZE,
        transform,
        ..default()
    }, YSort(-5.0)));
}

fn spawn_chunks_around_player(mut commands: Commands, textures: Res<GameTextures>, player_transform: Query<&Transform, With<LocalPlayer>>, mut chunk_manager: ResMut<ChunkManager>) {
    let player_transform = player_transform.single();
    let chunk_position = world_to_chunk_position(&player_transform.translation.xy());
    for x in (chunk_position.x - CHUNK_LOAD_SIZE.x as i32)..(chunk_position.x + CHUNK_LOAD_SIZE.x as i32) {
        for y in (chunk_position.y - CHUNK_LOAD_SIZE.y as i32)..(chunk_position.y + CHUNK_LOAD_SIZE.y as i32) {
            if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                spawn_chunk(&mut commands, &textures, IVec2::new(x, y));
            }
        }
    }
}

fn despawn_chunks(mut commands: Commands, player_transform: Query<&Transform, With<LocalPlayer>>, chunks: Query<(Entity, &Transform), With<Chunk>>, mut chunk_manager: ResMut<ChunkManager>) {
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

fn cleanup_chunks(mut commands: Commands, chunks: Query<Entity, With<Chunk>>, mut chunk_manager: ResMut<ChunkManager>) {
    for chunk in &chunks {
        commands.entity(chunk).despawn_recursive();
    }
    chunk_manager.spawned_chunks.clear();
}

pub fn world_to_chunk_position(camera_position: &Vec2) -> IVec2 {
    camera_position.as_ivec2() / (IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32) * IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32))
}