use bevy::app::App;
use bevy::math::Vec3Swizzles;
use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, IVec2, NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, States, Vec3, With};

use crate::client::state::GameState;
use crate::client::state::main_menu::home_screen::HomeScreenPlugin;
use crate::client::state::main_menu::menu_camera::MenuCameraPlugin;
use crate::client::texture::GameTextures;
use crate::client::world::{Chunk, spawn_chunk, world_to_chunk_position};

mod menu_camera;
mod home_screen;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_plugins((MenuCameraPlugin, HomeScreenPlugin))
            .add_systems(OnEnter(GameState::MainMenu), setup_screen)
            .add_systems(OnExit(GameState::MainMenu), cleanup_screen);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MainMenuState {
    None,
    #[default]
    HomeScreen
}

#[derive(Component)]
struct MenuChunk;

fn setup_screen(mut commands: Commands, textures: Res<GameTextures>) {
    spawn_empty_chunks(Vec3::ZERO, 2, &mut commands, &textures);
}

fn cleanup_screen(mut commands: Commands, mut menu_state: ResMut<NextState<MainMenuState>>, chunks: Query<Entity, With<MenuChunk>>) {
    menu_state.set(MainMenuState::None);

    for entity in &chunks {
        commands.entity(entity).despawn();
    }
}

fn spawn_empty_chunks(world_position: Vec3, radius: i32, mut commands: &mut Commands, textures: &GameTextures) {
    let chunk_position = world_to_chunk_position(&world_position.xy());
    for x in (chunk_position.x - radius)..(chunk_position.x + radius) {
        for y in (chunk_position.y - radius)..(chunk_position.y + radius) {
            let chunk = spawn_chunk(IVec2::new(x, y), Chunk::default(), &mut commands, &textures);
            commands.entity(chunk).insert(MenuChunk);
        }
    }
}