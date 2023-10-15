use std::time::Duration;

use bevy::app::App;
use bevy::asset::ChangeWatcher;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::{
    NoUserData, RapierConfiguration, RapierDebugRenderPlugin, RapierPhysicsPlugin,
};

use animation::SpriteAnimationPlugin;

use crate::asset::GameAssetPlugin;
use crate::client::camera::GameCameraPlugin;
use crate::client::state::StatePlugin;
use crate::client::y_sorting::YSortPlugin;
use crate::registry::atlas::TextureAtlasRegistry;
use crate::registry::chunk_data::TILE_SIZE;
use crate::registry::items::ItemRegistry;

mod animation;
mod camera;
mod state;
mod world;
mod y_sorting;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game | Dev".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(5)),
                    ..default()
                })
                .build(),
            AudioPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(TILE_SIZE.x),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(TextureAtlasRegistry::default())
        .insert_resource(ItemRegistry::default())
        //.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Numpad0)),
        )
        .add_plugins((
            GameAssetPlugin,
            YSortPlugin,
            SpriteAnimationPlugin,
            StatePlugin,
            GameCameraPlugin,
        )) // Core ingame features
        .add_systems(Startup, (init_client, configure_physics_engine));
    }
}

fn init_client() {
    info!("Initializing client...")
}

fn configure_physics_engine(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

mod items {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct RawItem {
        /*stack_size: u8,
        texture_atlas: String,
        texture_index: usize*/
    }
}
