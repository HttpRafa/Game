use bevy::math::UVec2;
use bevy_ecs_tilemap::prelude::TilemapTileSize;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 10.0, y: 10.0 };
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 10, y: 10 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 4,
    y: CHUNK_SIZE.y * 4
};
pub const CHUNK_LOAD_SIZE: UVec2 = UVec2 {
    x: 4,
    y: 4
};