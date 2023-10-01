mod client;

use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::client::ClientPlugin;

fn main() {
    App::new()
        .insert_resource(GridData {
            size: 10.0
        })
        .add_plugins(ClientPlugin)
        .run();
}

#[derive(Resource, InspectorOptions, Default, Reflect)]
#[reflect(Resource, InspectorOptions)]
pub struct GridData {
    size: f32
}