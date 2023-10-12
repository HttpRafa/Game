use crate::client::ClientPlugin;
use bevy::prelude::*;

mod asset;
mod client;
mod registry;

fn main() {
    App::new().add_plugins(ClientPlugin).run();
}
