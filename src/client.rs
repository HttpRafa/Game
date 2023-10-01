use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::client::animation::SpriteAnimationPlugin;

use crate::client::grid_cursor::GridCursorPlugin;
use crate::client::local_player::LocalPlayerPlugin;
use crate::client::remote_player::RemotePlayerPlugin;
use crate::client::ui::UIPlugin;

mod local_player;
mod ui;
mod grid_cursor;
mod animation;
mod remote_player;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game".into(),
                        ..default()
                    }),
                    ..default()
                })
                .build()
            )
            //.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
            .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
            .add_plugins((LocalPlayerPlugin, RemotePlayerPlugin, GridCursorPlugin, UIPlugin, SpriteAnimationPlugin));
    }
}