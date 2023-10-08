use bevy::app::App;
use bevy::prelude::{Plugin, Resource};
use crate::registry::items::Item;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInventory {
            slots: vec![]
        });
    }
}

#[derive(Resource)]
pub struct PlayerInventory {
    slots: Vec<ItemStack>
}

pub struct ItemStack {
    item: Item
}