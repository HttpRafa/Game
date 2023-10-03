use std::time::Duration;
use bevy::app::App;

use bevy::asset::Handle;
use bevy::ecs::prelude::{Component, Query, Res};
use bevy::prelude::{Plugin, Time, Timer, Update};
use bevy::sprite::{TextureAtlas, TextureAtlasSprite};

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

pub struct AnimationFrame {
    pub atlas_handle: Handle<TextureAtlas>,
    pub atlas_index: usize,
    pub duration: Duration,
}

impl Default for AnimationFrame {
    fn default() -> Self {
        Self {
            atlas_handle: Default::default(),
            atlas_index: 0,
            duration: Default::default(),
        }
    }
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
}

#[derive(Component)]
pub struct Animations {
    pub animations: Vec<Animation>,
}

#[derive(Default, Component)]
pub struct Animator {
    pub current_animation: usize,
    pub last_animation: usize,
    pub current_frame: usize,
    pub timer: Timer,
}

pub fn calc_animation_index(row: usize, colum: usize, row_size: usize) -> usize {
    colum + (row * row_size)
}

fn animate_sprite(mut query: Query<(&Animations, &mut Animator, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (animations, mut animator, mut atlas, mut sprite) in &mut query.iter_mut() {
        animator.timer.tick(time.delta());

        if !animator.timer.finished() && animator.last_animation == animator.current_animation {
            break;
        }
        animator.last_animation = animator.current_animation;

        if let Some(animation) = animations.animations.get(animator.current_animation) {
            animator.current_frame = if animator.current_frame + 1 < animation.frames.len() {
                animator.current_frame + 1
            } else {
                0
            };

            if let Some(frame) = animation.frames.get(animator.current_frame) {
                animator.timer.set_duration(frame.duration);
                animator.timer.reset();
                *atlas = frame.atlas_handle.clone();
                sprite.index = frame.atlas_index;
            }
        }
    }
}