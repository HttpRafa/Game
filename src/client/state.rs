use crate::client::state::ingame::InGamePlugin;
use crate::client::state::main_menu::MainMenuPlugin;
use bevy::app::App;
use bevy::prelude::{Plugin, States};

mod ingame;
mod main_menu;

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
    InGame,
}
