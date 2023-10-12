use crate::client::camera::MainCamera;
use crate::client::state::ingame::local_player::LocalPlayer;
use crate::client::state::GameState;
use bevy::app::App;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub struct InGameCameraPlugin;

impl Plugin for InGameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_player.run_if(in_state(GameState::InGame)))
            .add_systems(Update, scroll_camera.run_if(in_state(GameState::InGame)));
    }
}

const MIN_ZOOM: f32 = 0.75;
const MAX_ZOOM: f32 = 2.5;

fn scroll_camera(
    mut projection: Query<&mut OrthographicProjection, (With<Camera>, With<MainCamera>)>,
    mut scroll: EventReader<MouseWheel>,
) {
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

fn follow_player(
    mut camera_transform: Query<
        (&Camera, &GlobalTransform, &mut Transform),
        (With<MainCamera>, Without<LocalPlayer>),
    >,
    player_transform: Query<&Transform, (With<LocalPlayer>, Without<MainCamera>)>,
    window: Query<&Window>,
    time: Res<Time>,
) {
    let (camera, global_transform, mut camera_transform) = camera_transform.single_mut();
    let player_position = player_transform.single().translation;
    let window = window.single();

    let mut offset = Vec3::ZERO;
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(global_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let world_position = Vec3::new(world_position.x, world_position.y, 0.0);
        offset = (world_position - player_position) / 15.0;
    }

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_position + offset, time.delta_seconds() * 10.0);
}
