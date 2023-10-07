use bevy::app::App;
use bevy::prelude::{NextState, OnExit, Plugin, ResMut, States};
use crate::client::state::GameState;

use crate::client::state::main_menu::home_screen::HomeScreenPlugin;
use crate::client::state::main_menu::menu_camera::MenuCameraPlugin;

mod menu_camera;
mod home_screen;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_plugins((MenuCameraPlugin, HomeScreenPlugin))
            .add_systems(OnExit(GameState::MainMenu), cleanup_screen);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MainMenuState {
    None,
    #[default]
    HomeScreen
}

fn cleanup_screen(mut menu_state: ResMut<NextState<MainMenuState>>) {
    menu_state.set(MainMenuState::None);
}