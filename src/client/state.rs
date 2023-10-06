use bevy::app::App;
use bevy::prelude::{Plugin, States};
use crate::client::state::ingame::InGamePlugin;
use crate::client::state::main_menu::MainMenuPlugin;

mod main_menu;
mod ingame;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((MainMenuPlugin, InGamePlugin));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame
}