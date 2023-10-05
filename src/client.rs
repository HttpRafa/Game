use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::client::animation::SpriteAnimationPlugin;

use crate::client::grid_cursor::GridCursorPlugin;
use crate::client::ingame_ui::InGameUIPlugin;
use crate::client::local_player::LocalPlayerPlugin;
use crate::client::main_menu::MainMenuPlugin;
use crate::client::remote_player::RemotePlayerPlugin;
use crate::client::textures::TexturesPlugin;
use crate::client::world::WorldPlugin;
use crate::client::y_sorting::YSortPlugin;

mod local_player;
mod ingame_ui;
mod grid_cursor;
mod animation;
mod remote_player;
mod y_sorting;
mod world;
mod textures;
mod main_menu;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
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
            .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)))
            .add_plugins((TexturesPlugin, YSortPlugin, SpriteAnimationPlugin, WorldPlugin))
            .add_plugins((MainMenuPlugin))
            .add_plugins((LocalPlayerPlugin, RemotePlayerPlugin, GridCursorPlugin, InGameUIPlugin))
            .add_systems(Startup, init_client);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame
}

fn init_client() {
    info!("Initializing game client...")
}
