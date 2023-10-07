use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::{TileBundle, TilemapId, TilemapTexture, TilePos, TileStorage, TileTextureIndex};
use bevy_ecs_tilemap::TilemapBundle;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use rand::{Rng, thread_rng};
use crate::client::texture::{GameTextures, WORLD_GROUND_TILES};
use crate::client::y_sorting::YSort;
use crate::common::{CHUNK_SIZE, TILE_SIZE};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Chunk {
    data: ChunkData
}

#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct ChunkData;

pub fn world_to_chunk_position(camera_position: &Vec2) -> IVec2 {
    camera_position.as_ivec2() / (IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32) * IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32))
}

pub fn spawn_chunk(chunk_pos: IVec2, chunk: Chunk, commands: &mut Commands, textures: &GameTextures) -> Entity {
    let tilemap_entity = commands.spawn((chunk, Name::new(format!("Chunk {} {}", chunk_pos.x, chunk_pos.y)))).id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(thread_rng().gen_range(0..(WORLD_GROUND_TILES.columns * WORLD_GROUND_TILES.rows) as u32)),
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
    return tilemap_entity;
}