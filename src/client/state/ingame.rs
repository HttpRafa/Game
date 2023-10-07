use bevy::app::App;
use bevy::prelude::Plugin;
use crate::client::state::ingame::grid_cursor::GridCursorPlugin;
use crate::client::state::ingame::hud::HudPlugin;
use crate::client::state::ingame::local_player::LocalPlayerPlugin;
use crate::client::state::ingame::main_camera::InGameCameraPlugin;
use crate::client::state::ingame::remote_player::RemotePlayerPlugin;
use crate::client::state::ingame::remote_world::WorldPlugin;

mod remote_world;
mod remote_player;
mod local_player;
mod hud;
mod grid_cursor;
mod main_camera;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InGameCameraPlugin, WorldPlugin, LocalPlayerPlugin, RemotePlayerPlugin, GridCursorPlugin, HudPlugin));
    }
}