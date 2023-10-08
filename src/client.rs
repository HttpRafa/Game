use std::fs;
use std::time::Duration;

use bevy::app::App;
use bevy::asset::ChangeWatcher;
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use animation::SpriteAnimationPlugin;

use crate::client::asset::GameAssetPlugin;
use crate::client::camera::GameCameraPlugin;
use crate::client::state::StatePlugin;
use crate::client::y_sorting::YSortPlugin;
use crate::registry::items::Items;

mod state;
mod animation;
mod asset;
mod y_sorting;
mod world;
mod camera;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(5)),
                    ..default()
                })
                .build(), AudioPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0),
                RapierDebugRenderPlugin::default()
            ))
            .insert_resource(Items::default())
            //.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
            .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)))
            .add_plugins((GameAssetPlugin, YSortPlugin, SpriteAnimationPlugin, StatePlugin, GameCameraPlugin)) // Core ingame features
            .add_systems(Startup, (init_client, configure_physics_engine, load_items));
    }
}

fn init_client() {
    info!("Initializing client...")
}

fn configure_physics_engine(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn load_items(mut items: ResMut<Items>) {
    info!("Loading items...");
    match fs::read_dir("assets/data/items/") {
        Ok(files) => {
            for item in files {
                let item = item.unwrap();
                match fs::read_to_string(item.path()) {
                    Ok(content) => {
                        match toml::from_str(&content) {
                            Ok(item) => {
                                items.entities.push(item);
                            },
                            Err(error) => {
                                error!("Failed to parse item file: {} caused by {}", item.path().display(), error);
                            }
                        }
                    }
                    Err(error) => {
                        error!("Failed to read item file: {} caused by {}", item.path().display(), error);
                    }
                }
            }
        }
        Err(_) => {
            error!("Failed to read items directory");
        }
    }
}