use bevy::app::App;
use bevy::prelude::{Commands, OnEnter, OnExit, Plugin};
use crate::client::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

fn setup_menu(_commands: Commands) {

}

fn cleanup_menu(_commands: Commands) {

}