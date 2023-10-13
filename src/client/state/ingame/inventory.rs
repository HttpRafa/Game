use crate::registry::items::Item;
use crate::registry::player_data::{HOTBAR_SIZE, INVENTORY_SIZE};
use bevy::app::App;
use bevy::prelude::{Plugin, Reflect, Resource};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInventory {
            slots: Vec::with_capacity(INVENTORY_SIZE + HOTBAR_SIZE),
        });
    }
}

#[derive(Resource)]
pub struct PlayerInventory {
    pub slots: Vec<ItemStack>,
}

#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct ItemStack {
    pub item: Item,
    pub amount: u8,
}
