pub mod items {
    use bevy::prelude::{Reflect, Resource};
    use bevy::utils::HashMap;
    use bevy_inspector_egui::prelude::ReflectInspectorOptions;
    use bevy_inspector_egui::InspectorOptions;

    use crate::registry::atlas::GameAtlas;

    #[derive(InspectorOptions, Default, Reflect)]
    #[reflect(InspectorOptions)]
    pub struct Item {
        pub stack_size: u8,
        pub texture_atlas: GameAtlas,
        pub texture_index: usize,
    }

    impl Clone for Item {
        fn clone(&self) -> Self {
            Self {
                stack_size: self.stack_size,
                texture_atlas: self.texture_atlas.clone_weak(),
                texture_index: self.texture_index,
            }
        }
    }

    #[derive(Resource, Default)]
    pub struct ItemRegistry {
        pub entities: HashMap<String, Item>,
    }
}

pub mod atlas {
    use bevy::prelude::{Handle, Image, Reflect, Resource, TextureAtlas};
    use bevy::utils::HashMap;
    use bevy_inspector_egui::prelude::ReflectInspectorOptions;
    use bevy_inspector_egui::InspectorOptions;

    #[derive(Resource)]
    pub struct GameTextures {
        pub player_animations: GameAtlas,
        pub ui_inventory: GameAtlas,
        pub world_ground_tiles: GameAtlas,
    }

    #[derive(InspectorOptions, Default, Reflect)]
    #[reflect(InspectorOptions)]
    pub struct GameAtlas {
        pub tile_size_x: f32,
        pub tile_size_y: f32,
        pub columns: usize,
        pub rows: usize,
        pub image_handle: Handle<Image>,
        pub atlas_handle: Handle<TextureAtlas>,
    }

    impl GameAtlas {
        pub fn clone_weak(&self) -> Self {
            Self {
                tile_size_x: self.tile_size_x,
                tile_size_y: self.tile_size_y,
                columns: self.columns,
                rows: self.rows,
                image_handle: self.image_handle.clone_weak(),
                atlas_handle: self.atlas_handle.clone_weak(),
            }
        }
    }

    impl Clone for GameAtlas {
        fn clone(&self) -> Self {
            Self {
                tile_size_x: self.tile_size_x,
                tile_size_y: self.tile_size_y,
                columns: self.columns,
                rows: self.rows,
                image_handle: self.image_handle.clone(),
                atlas_handle: self.atlas_handle.clone(),
            }
        }
    }

    #[derive(Resource, Default)]
    pub struct TextureAtlasRegistry {
        pub entities: HashMap<String, GameAtlas>,
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
        y: CHUNK_SIZE.y * 3,
    };
    pub const CHUNK_LOAD_SIZE: UVec2 = UVec2 { x: 3, y: 3 };
}
