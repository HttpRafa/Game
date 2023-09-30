mod client;

use bevy::prelude::{App, Resource};
use crate::client::ClientPlugin;

fn main() {
    App::new()
        .insert_resource(GridData {
            size: 10.0
        })
        .add_plugins(ClientPlugin)
        .run();
}

#[derive(Resource)]
pub struct GridData {
    size: f32
}