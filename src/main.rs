use bevy::prelude::*;
use crate::client::ClientPlugin;

mod client;
mod registry;
mod asset;

fn main() {
    App::new()
        .add_plugins(ClientPlugin)
        .run();
}
