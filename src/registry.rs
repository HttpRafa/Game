pub mod items {
    use bevy::prelude::{Resource, TextureAtlas};
    use bevy::utils::HashMap;

    use crate::registry::atlas::LoadedAtlas;

    pub struct Item {
        pub stack_size: u8,
        pub texture_atlas: LoadedAtlas<TextureAtlas>,
        pub texture_index: usize
    }

    #[derive(Resource, Default)]
    pub struct ItemRegistry {
        pub entities: HashMap<String, Item>
    }
}

pub mod atlas {
    use bevy::asset::Asset;
    use bevy::prelude::{Handle, Image, Resource, TextureAtlas};
    use bevy::utils::HashMap;
    use serde::Deserialize;

    #[derive(Resource)]
    pub struct GameTextures {
        pub player_animations: LoadedAtlas<TextureAtlas>,
        pub ui_inventory: LoadedAtlas<TextureAtlas>,
        pub world_ground_tiles: LoadedAtlas<Image>
    }

    pub struct LoadedAtlas<T: Asset> {
        pub tile_size_x: f32,
        pub tile_size_y: f32,
        pub columns: usize,
        pub rows: usize,
        pub handle: Handle<T>
    }

    #[derive(Deserialize)]
    pub struct GameAtlas {
        pub texture: String,
        pub tile_size_x: f32,
        pub tile_size_y: f32,
        pub columns: usize,
        pub rows: usize,
    }

    #[derive(Resource, Default)]
    pub struct TextureAtlasRegistry {
        pub entities: HashMap<String, GameAtlas>
    }
}

pub mod audio {
    use bevy::prelude::{Handle, Resource};
    use bevy_kira_audio::AudioSource;

    #[derive(Resource)]
    pub struct BackgroundChannel;

    #[derive(Resource)]
    pub struct UIChannel;

    #[derive(Resource)]
    pub struct SoundEffectsChannel;

    #[derive(Resource)]
    pub struct GameSounds {
        pub ui_click: Handle<AudioSource>,
        pub ui_hover: Handle<AudioSource>,
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