use crate::client::state::ingame::grid_cursor::GridCursorPlugin;
use crate::client::state::ingame::hud::HudPlugin;
use crate::client::state::ingame::local_player::LocalPlayerPlugin;
use crate::client::state::ingame::main_camera::InGameCameraPlugin;
use crate::client::state::ingame::remote_player::RemotePlayerPlugin;
use crate::client::state::ingame::remote_world::WorldPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

use self::inventory::InventoryPlugin;

mod grid_cursor;
mod hud;
mod inventory;
mod local_player;
mod main_camera;
mod remote_player;
mod remote_world;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InGameCameraPlugin,
            WorldPlugin,
            InventoryPlugin,
            LocalPlayerPlugin,
            RemotePlayerPlugin,
            GridCursorPlugin,
            HudPlugin,
        ));
    }
}
