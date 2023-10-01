use bevy::app::App;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_sprite_animation::{AnimationNode, StartNode};
use bevy_sprite_animation::prelude::AnimationState;
use crate::client::player_animation::PlayerState;
use crate::client::remote_player::Player;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, scroll_camera)
            .register_type::<LocalPlayer>();
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct LocalPlayer {
    #[inspector(min = 0.0)]
    pub speed: f32
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

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create camera
    let mut camera = (
        Camera2dBundle::default(),
        MainCamera
    );
    camera.0.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 320.0,
        min_height: 180.0
    };

    // Setup animation
    let tree_handle: Handle<AnimationNode> = asset_server.load("animations/player_tree.nodetree");
    let start_node: Handle<AnimationNode> = asset_server.load("animations/player.node");
    let mut start = AnimationState::default();

    // Spawn local player
    commands.spawn((
        SpriteBundle::default(),
        LocalPlayer {
            speed: 33.3
        },
        Player,
        PlayerState::Idle,
        start,
        StartNode::from_handle(start_node),
        Name::new("Player")
    )).with_children(|commands| {
        commands.spawn(camera);
    });
}

fn player_movement(mut characters: Query<(&mut Transform, &LocalPlayer)>, input: Res<Input<KeyCode>>, time: Res<Time>) {
    let (mut transform, player) = characters.single_mut();
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    if input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement.y += 1.0;
    }
    if input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement.y -= 1.0;
    }
    if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement.x += 1.0;
    }
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement.x -= 1.0;
    }
    transform.translation += movement.normalize_or_zero() * player.speed * time.delta_seconds();
}