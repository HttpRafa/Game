use crate::client::camera::MainCamera;
use crate::client::state::GameState;
use crate::client::y_sorting::YSort;
use crate::registry::chunk_data::TILE_SIZE;
use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use super::InGameData;

pub struct GridCursorPlugin;

impl Plugin for GridCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_cursor)
            .add_systems(OnExit(GameState::InGame), cleanup_cursor)
            //.add_systems(Update, update_cursor.run_if(in_state(GameState::InGame)))
            .add_systems(Update, move_cursor.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct GridCursor;

fn move_cursor(
    mut cursors: Query<(&mut Transform, &mut Visibility), With<GridCursor>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Query<&Window>,
    ingame_data: Res<InGameData>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(mut world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        world_position.x = (world_position.x / TILE_SIZE.x).round() * TILE_SIZE.x;
        world_position.y = (world_position.y / TILE_SIZE.y).round() * TILE_SIZE.y;
        for (mut cursor, mut visibility) in &mut cursors {
            if ingame_data.screen_open {
                *visibility = Visibility::Hidden;
            } else {
                cursor.translation = Vec3::new(world_position.x, world_position.y, 0.0);
            }
        }
    }
}

/*fn update_cursor(
    cursors: Query<(&GridCursor, &Children), Changed<GridCursor>>,
    mut cursor_texture: Query<(
        &mut Visibility,
        &mut Handle<TextureAtlas>,
        &mut UiTextureAtlasImage,
    )>,
) {
    for (cursor, children) in &cursors {
        let (mut visibility, mut cursor_texture, mut image) =
            cursor_texture.get_mut(children[0]).unwrap();
        match &cursor.item {
            Some(item) => {
                image.index = item.texture_index;
                *cursor_texture = item.texture_atlas.atlas_handle.clone_weak();
                *visibility = Visibility::Visible;
            }
            None => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}*/

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
        GridCursor::default(),
        Name::new("Grid Cursor"),
        YSort(-2.5),
    ));
}

fn cleanup_cursor(mut commands: Commands, cursors: Query<Entity, With<GridCursor>>) {
    for cursor in &cursors {
        commands.entity(cursor).despawn_recursive();
    }
}
