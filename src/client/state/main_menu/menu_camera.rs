use bevy::app::App;
use bevy::prelude::{Camera2dBundle, Commands, Component, DespawnRecursiveExt, Entity, Name, OnEnter, OnExit, Plugin, Query, With};
use bevy::render::camera::ScalingMode;
use crate::client::state::GameState;

pub struct MenuCameraPlugin;

impl Plugin for MenuCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_camera)
            .add_systems(OnExit(GameState::MainMenu), cleanup_camera);
    }
}

#[derive(Component)]
struct MenuCamera;

fn setup_camera(mut commands: Commands) {
    // Create camera
    let mut camera = (
        Camera2dBundle::default(),
        MenuCamera,
        Name::new("Menu Camera")
    );
    camera.0.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 320.0,
        min_height: 180.0
    };

    commands.spawn(camera);
}

fn cleanup_camera(mut commands: Commands, cameras: Query<Entity, With<MenuCamera>>) {
    for entity in &cameras {
        commands.entity(entity).despawn_recursive();
    }
}