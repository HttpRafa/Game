mod player;
mod ui;

use bevy::app::App;
use bevy::core::Name;
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_toggle_active;
use bevy::math::Vec3;
use bevy::prelude::{AssetServer, Camera, Camera2dBundle, Commands, Component, GlobalTransform, ImagePlugin, KeyCode, Plugin, PluginGroup, Query, Res, Sprite, SpriteBundle, Startup, Transform, Update, Vec2, Window, WindowPlugin, With};
use bevy::render::camera::ScalingMode;
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::client::player::{Player, PlayerPlugin};
use crate::client::ui::UIPlugin;
use crate::GridData;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game".into(),
                        ..default()
                    }),
                    ..default()
                })
                .build()
            )
            .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
            .add_plugins((PlayerPlugin, UIPlugin))
            .add_systems(Startup, startup)
            .add_systems(Update, move_debug_cursor);
    }
}

#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct DebugCursor;

fn move_debug_cursor(mut cursor: Query<&mut Transform, With<DebugCursor>>, camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>, window: Query<&Window>, grid_data: Res<GridData>) {
    let (camera, camera_transform) = camera.single();
    let mut cursor = cursor.single_mut();
    let window = window.single();

    if let Some(mut world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        world_position.x = (world_position.x / grid_data.size).round() * grid_data.size;
        world_position.y = (world_position.y / grid_data.size).round() * grid_data.size;
        cursor.translation = Vec3::new(world_position.x, world_position.y, 0.0);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, grid_data: Res<GridData>) {
    let mut camera = (
        Camera2dBundle::default(),
        MainCamera
    );
    camera.0.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 320.0,
        min_height: 180.0
    };
    commands.spawn(camera);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(grid_data.size, grid_data.size)),
                ..default()
            },
            ..default()
        },
        DebugCursor
    ));

    let texture = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player {
            speed: 33.3
        },
        Name::new("Player")
    ));
}