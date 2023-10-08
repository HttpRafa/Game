pub mod items {
    use bevy::prelude::Resource;
    use serde::Deserialize;

    #[derive(Resource, Default)]
    pub struct Items {
        pub entities: Vec<Item>
    }

    #[derive(Deserialize)]
    pub struct Item {
        identifier: String,
        stack_size: u8,
        texture_index: usize
    }
}

pub mod chunk_data {
    use bevy::prelude::UVec2;
    use bevy_ecs_tilemap::prelude::TilemapTileSize;

    pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 10.0, y: 10.0 };
    pub const CHUNK_SIZE: UVec2 = UVec2 { x: 10, y: 10 };
    pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
        x: CHUNK_SIZE.x * 3,
        y: CHUNK_SIZE.y * 3
    };
    pub const CHUNK_LOAD_SIZE: UVec2 = UVec2 {
        x: 3,
        y: 3
    };
}