use std::time::Duration;

use bevy::app::App;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::client::animation::{Animation, AnimationFrame, Animations, Animator, calc_animation_index};
use crate::client::state::GameState;
use crate::client::state::ingame::remote_player::Player;
use crate::client::y_sorting::YSort;
use crate::registry::atlas::GameTextures;
use crate::registry::chunk_data::TILE_SIZE;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(OnExit(GameState::InGame), cleanup_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::InGame)))
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

#[derive(Bundle)]
struct LocalPlayerBundle {
    y_sort: YSort,
    sprite_bundle: SpriteSheetBundle,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    local_player: LocalPlayer,
    player: Player,
    animator: Animator,
    animations: Animations,
    name: Name
}

fn setup_player(mut commands: Commands, textures: Res<GameTextures>) {
    // Spawn local player
    commands.spawn(LocalPlayerBundle {
        y_sort: YSort::default(),
        sprite_bundle: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(TILE_SIZE.x * 1.5, TILE_SIZE.y * 1.5)),
                ..default()
            },
            texture_atlas: textures.player_animations.handle.clone(),
            ..default()
        },
        rigid_body: RigidBody::Dynamic,
        collider: Collider::capsule_y(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0),
        velocity: Velocity::zero(),
        local_player: LocalPlayer {
            speed: 40.0,
            direction: Default::default(),
        },
        player: Default::default(),
        animator: Default::default(),
        animations: Animations {
            animations: vec![
                // Idle animation
                gen_animation(&textures.player_animations.handle, 0, 2, 0.55),
                // Walk right animation
                gen_animation(&textures.player_animations.handle, 1, 3, 0.25),
                // Walk left animation
                gen_animation(&textures.player_animations.handle, 2, 3, 0.25),
                // Walk up animation
                gen_animation(&textures.player_animations.handle, 3, 3, 0.25),
                // Walk down animation
                gen_animation(&textures.player_animations.handle, 4, 3, 0.25),
                // Walk up right animation
                gen_animation(&textures.player_animations.handle, 5, 3, 0.25),
                // Walk up left animation
                gen_animation(&textures.player_animations.handle, 6, 3, 0.25),
                // Walk down right animation
                gen_animation(&textures.player_animations.handle, 7, 3, 0.25),
                // Walk down left animation
                gen_animation(&textures.player_animations.handle, 8, 3, 0.25),
            ]
        },
        name: Name::new("LocalPlayer"),
    });
}

fn cleanup_player(mut commands: Commands, players: Query<Entity, With<LocalPlayer>>) {
    for entity in &players {
        commands.entity(entity).despawn_recursive();
    }
}

fn gen_animation(texture: &Handle<TextureAtlas>, row: usize, colum_amount: usize, duration: f32) -> Animation {
    let mut frames: Vec<AnimationFrame> = vec![];

    for colum in 0..colum_amount {
        frames.push(AnimationFrame {
            atlas_handle: texture.clone(),
            atlas_index: calc_animation_index(row, colum, colum_amount),
            duration: Duration::from_secs_f32(duration),
            ..default()
        });
    }

    Animation {
        frames
    }
}

fn player_movement(mut characters: Query<(&mut Velocity, &mut Animator, &mut LocalPlayer)>, keyboard_input: Res<Input<KeyCode>>, gamepads: Res<Gamepads>, gamepad_axes: Res<Axis<GamepadAxis>>) {
    let (mut velocity, mut animator, mut player) = characters.single_mut();
    let mut movement = Vec2::new(0.0, 0.0);

    for gamepad in gamepads.iter() {
        movement.x = gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap();
        movement.y = gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)).unwrap();
    }

    if movement == Vec2::ZERO {
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

    velocity.linvel = movement * player.speed;
    player.direction = movement;
    
    if movement == Vec2::ZERO {
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