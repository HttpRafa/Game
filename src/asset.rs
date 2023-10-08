use std::fs;

use bevy::app::App;
use bevy::asset::Asset;
use bevy::prelude::{error, Handle, Image, info, Plugin, Resource, TextureAtlas};
use bevy::utils::HashMap;
use bevy_kira_audio::AudioSource;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::asset::audio::GameAssetAudioPlugin;
use crate::asset::textures::GameAssetTexturePlugin;

pub struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameAssetAudioPlugin, GameAssetTexturePlugin));
    }
}

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

mod textures {
    use bevy::app::App;
    use bevy::prelude::{Assets, AssetServer, Commands, Handle, Image, info, Plugin, PreStartup, Res, ResMut, Startup, TextureAtlas, Vec2};

    use crate::asset::{GameAtlas, GameTextures, load_instances_from_file, LoadedAtlas};
    use crate::registry::atlas::TextureAtlasRegistry;

    pub struct GameAssetTexturePlugin;

    impl Plugin for GameAssetTexturePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(PreStartup, load_atlases)
                .add_systems(Startup, init_textures);
        }
    }

    fn load_atlases(mut registry: ResMut<TextureAtlasRegistry>) {
        info!("Loading texture atlases...");
        load_instances_from_file("assets/data/atlas/", &mut registry.entities);
    }

    fn init_textures(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, registry: Res<TextureAtlasRegistry>) {
        let registry = &registry.entities;
        commands.insert_resource(GameTextures {
            player_animations: registry["player_animations"].create_atlas_loaded(&asset_server, &mut texture_atlases),
            ui_inventory: registry["ui_inventory"].create_atlas_loaded(&asset_server, &mut texture_atlases),
            world_ground_tiles: registry["ground_tiles"].create_loaded(&asset_server)
        });
    }

    impl GameAtlas {
        fn create_loaded(&self, asset_server: &Res<AssetServer>) -> LoadedAtlas<Image> {
            LoadedAtlas {
                tile_size_x: self.tile_size_x,
                tile_size_y: self.tile_size_y,
                columns: self.columns,
                rows: self.rows,
                handle: self.create_handle(asset_server),
            }
        }

        fn create_atlas_loaded(&self, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> LoadedAtlas<TextureAtlas> {
            LoadedAtlas {
                tile_size_x: self.tile_size_x,
                tile_size_y: self.tile_size_y,
                columns: self.columns,
                rows: self.rows,
                handle: self.create_atlas_handle(asset_server, texture_atlases),
            }
        }

        fn create_handle(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
            asset_server.load(&self.texture)
        }

        fn create_atlas_handle(&self, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Handle<TextureAtlas> {
            let texture_atlas = TextureAtlas::from_grid(asset_server.load(&self.texture), Vec2::new(self.tile_size_x, self.tile_size_y), self.columns, self.rows, None, None);
            texture_atlases.add(texture_atlas)
        }
    }
}

mod audio {
    use bevy::app::App;
    use bevy::prelude::{AssetServer, Commands, info, Plugin, PreStartup, Res};
    use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl};

    use crate::asset::{BackgroundChannel, GameSounds, SoundEffectsChannel, UIChannel};

    pub struct GameAssetAudioPlugin;

    impl Plugin for GameAssetAudioPlugin {
        fn build(&self, app: &mut App) {
            app.add_audio_channel::<BackgroundChannel>()
                .add_audio_channel::<UIChannel>()
                .add_audio_channel::<SoundEffectsChannel>()
                .add_systems(PreStartup, (load_audio, init_audio_channels));
        }
    }

    fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
        info!("Loading audio files...");
        commands.insert_resource(GameSounds {
            ui_click: asset_server.load("audio/ui/click.ogg"),
            ui_hover: asset_server.load("audio/ui/hover.ogg")
        });
    }

    pub fn init_audio_channels(background_channel: Res<AudioChannel<BackgroundChannel>>, ui_channel: Res<AudioChannel<UIChannel>>, fx_channel: Res<AudioChannel<SoundEffectsChannel>>) {
        background_channel.set_volume(0.1);
        ui_channel.set_volume(0.1);
        fx_channel.set_volume(0.1);
    }
}

fn load_instances_from_file<T: DeserializeOwned>(path: &str, registry: &mut HashMap<String, T>) {
    match fs::read_dir(path) {
        Ok(files) => {
            for data_file in files {
                let data_file = data_file.unwrap();
                match fs::read_to_string(data_file.path()) {
                    Ok(content) => {
                        match toml::from_str(&content) {
                            Ok(data) => {
                                let identifier = data_file.path().file_stem().unwrap().to_str().unwrap().to_owned();
                                info!("Loaded game data[{}]", identifier);
                                registry.insert(identifier, data);
                            }
                            Err(error) => {
                                error!("Failed to parse data file: {} caused by {}", data_file.path().display(), error);
                            }
                        }
                    }
                    Err(error) => {
                        error!("Failed to read data file: {} caused by {}", data_file.path().display(), error);
                    }
                }
            }
        }
        Err(_) => {
            error!("Failed to read data directory");
        }
    }
}