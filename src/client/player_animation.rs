use bevy::app::App;
use bevy::prelude::{Changed, Component, Handle, IntoSystemConfigs, Plugin, Query, Reflect, Resource, Update, With};
use bevy_sprite_animation::prelude::{AnimationSet, AnimationState, Attribute, MatchNode};
use bevy_sprite_animation::{AnimationNode, SpriteAnimationPlugin};
use serde::{Deserialize, Serialize};
use crate::client::remote_player::Player;

pub struct PlayerAnimation;

impl Plugin for PlayerAnimation {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpriteAnimationPlugin::<5>)
            .add_systems(Update, (player_state_update.before(AnimationSet::Update), player_update_state.after(AnimationSet::Update)))
            .register_type::<PlayerState>()
            .register_type::<MatchNode<PlayerState>>();
    }
}

#[derive(Debug, Component, Hash, PartialEq, Eq, Clone, Copy, Reflect, Serialize, Deserialize, PartialOrd, Ord)]
pub enum PlayerState {
    Idle,
    Walking
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

fn player_state_update(mut players: Query<(&mut AnimationState, &PlayerState), (With<Player>, Changed<PlayerState>)>) {
    let attribute = Attribute::new_attribute("PlayerState");
    for (mut state, name) in players.iter_mut() {
        state.set_attribute(attribute.clone(), *name)
    }
}

fn player_update_state(mut players: Query<(&AnimationState, &mut PlayerState), With<Player>>) {
    let attribute = Attribute::from_str("PlayerState");
    for (state, mut name) in players.iter_mut() {
        if state.changed(&attribute) {
            *name = *state.attribute(&attribute);
        }
    }
}