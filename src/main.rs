use bevy::prelude::*;
use crate::client::ClientPlugin;

mod client;
mod common;

fn main() {
    App::new()
        .add_plugins(ClientPlugin)
        .run();
}