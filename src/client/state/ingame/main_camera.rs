use bevy::app::App;
use bevy::core::Name;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;

use crate::client::state::GameState;
use crate::client::state::ingame::local_player::LocalPlayer;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_camera)
            .add_systems(OnExit(GameState::InGame), cleanup_camera)
            .add_systems(Update, follow_player.run_if(in_state(GameState::InGame)))
            .add_systems(Update, scroll_camera.run_if(in_state(GameState::InGame)))
            .register_type::<MainCamera>();
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct MainCamera;

const MIN_ZOOM: f32 = 0.75;
const MAX_ZOOM: f32 = 2.5;

fn scroll_camera(mut projection: Query<&mut OrthographicProjection, (With<Camera>, With<MainCamera>)>, mut scroll: EventReader<MouseWheel>) {
    let mut projection = projection.single_mut();
    let mut movement = 0.0;
    for event in scroll.iter() {
        match event.unit {
            MouseScrollUnit::Line => {
                movement -= event.y;
            }
            _ => {}
        }
    }
    projection.scale = (projection.scale + movement / 7.5).clamp(MIN_ZOOM, MAX_ZOOM);
}

fn follow_player(mut camera_transform: Query<(&Camera, &GlobalTransform, &mut Transform), (With<MainCamera>, Without<LocalPlayer>)>,
                 player_transform: Query<&Transform, (With<LocalPlayer>, Without<MainCamera>)>,
                 window: Query<&Window>,
                 time: Res<Time>
) {
    let (camera, global_transform, mut camera_transform) = camera_transform.single_mut();
    let player_position = player_transform.single().translation;
    let window = window.single();

    let mut offset = Vec3::ZERO;
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(global_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        let world_position = Vec3::new(world_position.x, world_position.y, 0.0);
        offset = (world_position - player_position) / 15.0;
    }

    camera_transform.translation = camera_transform.translation.lerp(player_position + offset, time.delta_seconds() * 10.0);
}

fn setup_camera(mut commands: Commands) {
    // Create camera
    let mut camera = (
        Camera2dBundle::default(),
        MainCamera,
        Name::new("Main Camera")
    );
    camera.0.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 320.0,
        min_height: 180.0
    };

    commands.spawn(camera);
}

fn cleanup_camera(mut commands: Commands, cameras: Query<Entity, With<MainCamera>>) {
    for entity in &cameras {
        commands.entity(entity).despawn_recursive();
    }
}