use std::time::Duration;
use bevy::app::App;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::client::animation::{Animation, AnimationFrame, Animations, Animator};
use crate::client::remote_player::Player;
use crate::client::y_sorting::YSort;
use crate::common::TILE_SIZE;

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

#[derive(Bundle)]
struct LocalPlayerBundle {
    y_sort: YSort,
    sprite_bundle: SpriteSheetBundle,
    local_player: LocalPlayer,
    player: Player,
    animator: Animator,
    animations: Animations,
    name: Name
}

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

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Create camera
    let mut camera = (
        Camera2dBundle::default(),
        MainCamera
    );
    camera.0.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 320.0,
        min_height: 180.0
    };

    // Load idle texture atlas
    let idle_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("animations/player/idle.png"), Vec2::new(16.0, 16.0), 5, 1, None, None));
    let walk_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("animations/player/walking.png"), Vec2::new(16.0, 16.0), 3, 2, None, None));

    // Spawn local player
    commands.spawn(LocalPlayerBundle {
        y_sort: YSort::default(),
        sprite_bundle: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(TILE_SIZE.x * 1.5, TILE_SIZE.y * 1.5)),
                ..default()
            },
            texture_atlas: idle_texture_atlas.clone(),
            ..default()
        },
        local_player: LocalPlayer {
            speed: 33.3
        },
        player: Default::default(),
        animator: Default::default(),
        animations: Animations {
            animations: vec![
                // Idle animation
                Animation {
                    frames: vec![
                        AnimationFrame {
                            atlas_handle: idle_texture_atlas.clone(),
                            atlas_index: 0,
                            duration: Duration::from_secs_f32(0.55),
                            ..default()
                        },
                        AnimationFrame {
                            atlas_handle: idle_texture_atlas.clone(),
                            atlas_index: 1,
                            duration: Duration::from_secs_f32(0.55),
                            ..default()
                        }
                    ]
                },
                // Walk right animation
                Animation {
                    frames: vec![
                        AnimationFrame {
                            atlas_handle: walk_texture_atlas.clone(),
                            atlas_index: 0,
                            duration: Duration::from_secs_f32(0.25),
                            ..default()
                        },
                        AnimationFrame {
                            atlas_handle: walk_texture_atlas.clone(),
                            atlas_index: 1,
                            duration: Duration::from_secs_f32(0.25),
                            ..default()
                        },
                        AnimationFrame {
                            atlas_handle: walk_texture_atlas.clone(),
                            atlas_index: 2,
                            duration: Duration::from_secs_f32(0.25),
                            ..default()
                        }
                    ]
                },
                // Walk left animation
                Animation {
                    frames: vec![
                        AnimationFrame {
                            atlas_handle: walk_texture_atlas.clone(),
                            atlas_index: 3,
                            duration: Duration::from_secs_f32(0.25),
                            ..default()
                        },
                        AnimationFrame {
                            atlas_handle: walk_texture_atlas.clone(),
                            atlas_index: 4,
                            duration: Duration::from_secs_f32(0.25),
                            ..default()
                        },
                        AnimationFrame {
                            atlas_handle: walk_texture_atlas.clone(),
                            atlas_index: 5,
                            duration: Duration::from_secs_f32(0.25),
                            ..default()
                        }
                    ]
                }
            ]
        },
        name: Name::new("LocalPlayer"),
    }).with_children(|commands| {
        commands.spawn(camera);
    });
}

fn player_movement(mut characters: Query<(&mut Transform, &mut Animator, &LocalPlayer)>, input: Res<Input<KeyCode>>, time: Res<Time>) {
    let (mut transform, mut animator, player) = characters.single_mut();
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

    if movement.x == 0.0 {
        // Is standing still

        // If not in Idle animation
        if animator.current_animation != 0 {
            animator.current_animation = 0;
            animator.current_frame = 0;
        }
        return;
    }
    if movement.x > 0.0 {
        // Moving to the right

        // If not in walking right animation
        if animator.current_animation != 1 {
            animator.current_animation = 1;
            animator.current_frame = 0;
        }
    } else {
        // Moving to the left

        // If not in walking left animation
        if animator.current_animation != 2 {
            animator.current_animation = 2;
            animator.current_frame = 0;
        }
    }
}