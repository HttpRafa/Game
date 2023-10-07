use bevy::app::App;
use bevy::prelude::{OnEnter, OrthographicProjection, Plugin, Query, With};

use crate::client::camera::MainCamera;
use crate::client::state::GameState;

pub struct MenuCameraPlugin;

impl Plugin for MenuCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_camera);
    }
}

fn setup_camera(mut projection: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut projection = projection.single_mut();
    projection.scale = 1.0;
}