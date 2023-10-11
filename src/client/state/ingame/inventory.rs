use bevy::app::App;
use bevy::prelude::{Plugin, Reflect, Resource};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::registry::items::Item;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInventory {
            //slots: vec![]
        });
    }
}

#[derive(Resource)]
pub struct PlayerInventory {
    //slots: Vec<ItemStack>
}

#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct ItemStack {
    pub item: Item,
    pub amount: u8
}