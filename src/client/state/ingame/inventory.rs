use bevy::app::App;
use bevy::prelude::Plugin;

use self::player_inventory::PlayerInventoryPlugin;

pub mod player_inventory;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerInventoryPlugin);
    }
}
