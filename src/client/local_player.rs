use std::time::Duration;

use bevy::app::App;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;

use crate::client::animation::{Animation, AnimationFrame, Animations, Animator, calc_animation_index};
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
    pub speed: f32,
    pub direction: Vec2,
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

const TEXTURE_COLUMN_AMOUNT: usize = 3;

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
    let walk_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("animations/player/walking.png"), Vec2::new(16.0, 16.0), 3, 8, None, None));

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
            speed: 40.0,
            direction: Default::default(),
        },
        player: Default::default(),
        animator: Default::default(),
        animations: Animations {
            animations: vec![
                // Idle animation
                gen_animation(&idle_texture_atlas, 0, 2, 0.55),
                // Walk right animation
                gen_animation(&walk_texture_atlas, 0, 3, 0.25),
                // Walk left animation
                gen_animation(&walk_texture_atlas, 1, 3, 0.25),
                // Walk up animation
                gen_animation(&walk_texture_atlas, 2, 3, 0.25),
                // Walk down animation
                gen_animation(&walk_texture_atlas, 3, 3, 0.25),
                // Walk up right animation
                gen_animation(&walk_texture_atlas, 4, 3, 0.25),
                // Walk up left animation
                gen_animation(&walk_texture_atlas, 5, 3, 0.25),
                // Walk down right animation
                gen_animation(&walk_texture_atlas, 6, 3, 0.25),
                // Walk down left animation
                gen_animation(&walk_texture_atlas, 7, 3, 0.25),
            ]
        },
        name: Name::new("LocalPlayer"),
    }).with_children(|commands| {
        commands.spawn(camera);
    });
}

fn gen_animation(texture: &Handle<TextureAtlas>, row: usize, colum_amount: usize, duration: f32) -> Animation {
    let mut frames: Vec<AnimationFrame> = vec![];

    for colum in 0..colum_amount {
        frames.push(AnimationFrame {
            atlas_handle: texture.clone(),
            atlas_index: calc_animation_index(row, colum, TEXTURE_COLUMN_AMOUNT),
            duration: Duration::from_secs_f32(duration),
            ..default()
        });
    }

    Animation {
        frames
    }
}

fn player_movement(mut characters: Query<(&mut Transform, &mut Animator, &mut LocalPlayer)>, keyboard_input: Res<Input<KeyCode>>, gamepads: Res<Gamepads>, gamepad_axes: Res<Axis<GamepadAxis>>, time: Res<Time>) {
    let (mut transform, mut animator, mut player) = characters.single_mut();
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    for gamepad in gamepads.iter() {
        movement.x = gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap();
        movement.y = gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)).unwrap();
    }

    if movement == Vec3::ZERO {
        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            movement.y += 1.0;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            movement.y -= 1.0;
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            movement.x += 1.0;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            movement.x -= 1.0;
        }
    }

    if movement.length() > 1.0 {
        movement = movement.normalize_or_zero();
    }

    transform.translation += movement * player.speed * time.delta_seconds();
    player.direction = Vec2::new(movement.x, movement.y).normalize_or_zero();
    
    if movement == Vec3::ZERO {
        // Is standing still

        // If not in Idle animation
        change_animation(0, &mut animator);
        return;
    }
    if movement.x == 0.0 {
        if movement.y > 0.0 {
            // Moving up

            // If not in walking up animation
            change_animation(3, &mut animator);
        } else {
            // Moving down

            // If not in walking down animation
            change_animation(4, &mut animator);
        }

    } else if movement.x > 0.0 {
        if movement.y == 0.0 {
            // Moving to the right

            // If not in walking right animation
            change_animation(1, &mut animator);
        } else if movement.y > 0.0 {
            // Moving up right

            // If not in walking up right animation
            change_animation(5, &mut animator);
        } else {
            // Moving down right

            // If not in walking down right animation
            change_animation(7, &mut animator);
        }
    } else {
        if movement.y == 0.0 {
            // Moving to the left

            // If not in walking left animation
            change_animation(2, &mut animator);
        } else if movement.y > 0.0 {
            // Moving up left

            // If not in walking up left animation
            change_animation(6, &mut animator);
        } else {
            // Moving down left

            // If not in walking down left animation
            change_animation(8, &mut animator);
        }
    }
}

fn change_animation(animation: usize, animator: &mut Animator) {
    if animator.current_animation != animation {
        animator.current_animation = animation;
        animator.current_frame = 0;
    }
}