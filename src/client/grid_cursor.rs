use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::client::local_player::MainCamera;

use crate::GridData;

pub struct GridCursorPlugin;

impl Plugin for GridCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor)
            .add_systems(Update, move_cursor);
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct GridCursor;

fn spawn_cursor(mut commands: Commands, grid_data: Res<GridData>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(grid_data.size, grid_data.size)),
                ..default()
            },
            ..default()
        },
        GridCursor,
        Name::new("Debug Grid Cursor")
    ));
}

fn move_cursor(mut cursors: Query<&mut Transform, With<GridCursor>>, camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>, window: Query<&Window>, grid_data: Res<GridData>) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(mut world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        world_position.x = (world_position.x / grid_data.size).round() * grid_data.size;
        world_position.y = (world_position.y / grid_data.size).round() * grid_data.size;
        for mut cursor in &mut cursors {
            cursor.translation = Vec3::new(world_position.x, world_position.y, 0.0);
        }
    }
}