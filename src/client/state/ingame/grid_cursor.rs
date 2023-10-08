use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::client::camera::MainCamera;
use crate::client::state::GameState;
use crate::client::y_sorting::YSort;
use crate::registry::chunk_data::TILE_SIZE;

pub struct GridCursorPlugin;

impl Plugin for GridCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_cursor)
            .add_systems(OnExit(GameState::InGame), cleanup_cursor)
            .add_systems(Update, move_cursor.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct GridCursor;

fn move_cursor(mut cursors: Query<&mut Transform, With<GridCursor>>, camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>, window: Query<&Window>) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(mut world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        world_position.x = (world_position.x / TILE_SIZE.x).round() * TILE_SIZE.x;
        world_position.y = (world_position.y / TILE_SIZE.y).round() * TILE_SIZE.y;
        for mut cursor in &mut cursors {
            cursor.translation = Vec3::new(world_position.x, world_position.y, 0.0);
        }
    }
}

fn setup_cursor(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE.x, TILE_SIZE.y)),
                color: Color::rgba(1.0, 1.0, 1.0, 0.05),
                ..default()
            },
            ..default()
        },
        GridCursor,
        Name::new("Grid Cursor"),
        YSort(-2.5)
    ));
}

fn cleanup_cursor(mut commands: Commands, cursors: Query<Entity, With<GridCursor>>) {
    for cursor in &cursors {
        commands.entity(cursor).despawn_recursive();
    }
}