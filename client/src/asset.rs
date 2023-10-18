use std::fs;

use bevy::app::App;
use bevy::prelude::{error, Plugin};
use bevy::utils::HashMap;
use serde::de::DeserializeOwned;

use crate::asset::audio::AudioPlugin;
use crate::asset::items::ItemsPlugin;
use crate::asset::textures::TexturesPlugin;

pub struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, TexturesPlugin, ItemsPlugin));
    }
}

mod items {
    use bevy::app::App;
    use bevy::prelude::{info, Plugin, Res, ResMut, Startup};
    use bevy::utils::HashMap;
    use serde::Deserialize;

    use crate::asset::load_data_from_files;
    use crate::registry::atlas::TextureAtlasRegistry;
    use crate::registry::items::{Item, ItemRegistry};

    pub struct ItemsPlugin;

    impl Plugin for ItemsPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, load_items);
        }
    }

    #[derive(Deserialize)]
    struct RawItem {
        stack_size: u8,
        texture_atlas: String,
        texture_index: usize,
    }

    fn load_items(mut registry: ResMut<ItemRegistry>, atlases: Res<TextureAtlasRegistry>) {
        let atlases = &atlases.entities;
        info!("--- [Registering items] ---");
        let mut raw_items: HashMap<String, RawItem> = HashMap::new();
        load_data_from_files("assets/data/items/", &mut raw_items);
        for (identifier, item) in raw_items {
            info!("Registering item \"{}\"", identifier);
            registry.entities.insert(
                identifier,
                Item {
                    stack_size: item.stack_size,
                    texture_atlas: atlases[&item.texture_atlas].clone(),
                    texture_index: item.texture_index,
                },
            );
        }
        info!("{} items were registered", registry.entities.len());
    }
}

mod textures {
    use bevy::app::App;
    use bevy::prelude::{
        info, AssetServer, Assets, Commands, Handle, Image, Plugin, PreStartup, Res, ResMut,
        Startup, TextureAtlas, Vec2,
    };
    use bevy::utils::HashMap;
    use serde::Deserialize;

    use crate::asset::load_data_from_files;
    use crate::registry::atlas::{GameAtlas, GameTextures, TextureAtlasRegistry};

    pub struct TexturesPlugin;

    impl Plugin for TexturesPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(PreStartup, load_atlases)
                .add_systems(Startup, init_textures);
        }
    }

    #[derive(Deserialize)]
    struct RawGameAtlas {
        pub texture: String,
        pub tile_size_x: f32,
        pub tile_size_y: f32,
        pub columns: usize,
        pub rows: usize,
    }

    fn load_atlases(
        mut registry: ResMut<TextureAtlasRegistry>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        info!("--- [Loading texture atlases] ---");
        let mut raw_atlases: HashMap<String, RawGameAtlas> = HashMap::new();
        load_data_from_files("assets/data/atlas/", &mut raw_atlases);
        for (identifier, atlas) in raw_atlases {
            info!("Loading atlas \"{}\"", identifier);
            registry.entities.insert(
                identifier,
                atlas.create_atlas_loaded(&asset_server, &mut texture_atlases),
            );
        }
        info!("{} atlases were loaded", registry.entities.len());
    }

    fn init_textures(mut commands: Commands, registry: Res<TextureAtlasRegistry>) {
        let registry = &registry.entities;
        commands.insert_resource(GameTextures {
            player_animations: registry["player_animations"].clone(),
            ui_inventory: registry["ui_inventory"].clone(),
            world_ground_tiles: registry["ground_tiles"].clone(),
        });
    }

    impl RawGameAtlas {
        fn create_atlas_loaded(
            &self,
            asset_server: &Res<AssetServer>,
            texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        ) -> GameAtlas {
            let handles = self.create_atlas_handle(asset_server, texture_atlases);
            GameAtlas {
                tile_size_x: self.tile_size_x,
                tile_size_y: self.tile_size_y,
                columns: self.columns,
                rows: self.rows,
                image_handle: handles.0,
                atlas_handle: handles.1,
            }
        }

        fn create_atlas_handle(
            &self,
            asset_server: &Res<AssetServer>,
            texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        ) -> (Handle<Image>, Handle<TextureAtlas>) {
            let image_handle = asset_server.load(&self.texture);
            let texture_atlas = TextureAtlas::from_grid(
                image_handle.clone(),
                Vec2::new(self.tile_size_x, self.tile_size_y),
                self.columns,
                self.rows,
                None,
                None,
            );
            (image_handle, texture_atlases.add(texture_atlas))
        }
    }
}

mod audio {
    use bevy::app::App;
    use bevy::prelude::{info, AssetServer, Commands, Plugin, PreStartup, Res};
    use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl};

    use crate::registry::audio::{BackgroundChannel, GameSounds, SoundEffectsChannel, UIChannel};

    pub struct AudioPlugin;

    impl Plugin for AudioPlugin {
        fn build(&self, app: &mut App) {
            app.add_audio_channel::<BackgroundChannel>()
                .add_audio_channel::<UIChannel>()
                .add_audio_channel::<SoundEffectsChannel>()
                .add_systems(PreStartup, (load_audio, init_audio_channels));
        }
    }

    fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("--- [Loading audio files] ---");
        commands.insert_resource(GameSounds {
            ui_click: asset_server.load("audio/ui/click.ogg"),
            ui_hover: asset_server.load("audio/ui/hover.ogg"),
        });
        info!("{} audio files were loaded", 2);
    }

    pub fn init_audio_channels(
        background_channel: Res<AudioChannel<BackgroundChannel>>,
        ui_channel: Res<AudioChannel<UIChannel>>,
        fx_channel: Res<AudioChannel<SoundEffectsChannel>>,
    ) {
        background_channel.set_volume(0.1);
        ui_channel.set_volume(0.1);
        fx_channel.set_volume(0.1);
    }
}

trait Identifiable {
    fn namespace() -> String;
    fn identifier() -> String;
}

fn load_data_from_files<T: DeserializeOwned>(path: &str, registry: &mut HashMap<String, T>) {
    let path = std::env::current_dir().unwrap().join(path);
    match fs::read_dir(&path) {
        Ok(files) => {
            for data_file in files {
                let data_file = data_file.unwrap();
                match fs::read_to_string(data_file.path()) {
                    Ok(content) => match toml::from_str(&content) {
                        Ok(data) => {
                            let identifier = data_file
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_owned();
                            registry.insert(identifier, data);
                        }
                        Err(error) => {
                            error!(
                                "Failed to parse data file: {} caused by {}",
                                data_file.path().display(),
                                error
                            );
                        }
                    },
                    Err(error) => {
                        error!(
                            "Failed to read data file: {} caused by {}",
                            data_file.path().display(),
                            error
                        );
                    }
                }
            }
        }
        Err(_) => {
            error!("Failed to read data directory {}", path.display());
        }
    }
}
