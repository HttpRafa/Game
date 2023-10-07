use std::time::Duration;
use bevy::app::App;
use bevy::asset::ChangeWatcher;
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use animation::SpriteAnimationPlugin;
use crate::client::camera::GameCameraPlugin;
use crate::client::state::StatePlugin;
use crate::client::asset::GameAssetPlugin;
use crate::client::y_sorting::YSortPlugin;

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
                .build(), AudioPlugin
            ))
            //.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
            .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)))
            .add_plugins((GameAssetPlugin, YSortPlugin, SpriteAnimationPlugin, StatePlugin, GameCameraPlugin)) // Core ingame features
            .add_systems(Startup, init_client);
    }
}

fn init_client() {
    info!("Initializing client...")
}
