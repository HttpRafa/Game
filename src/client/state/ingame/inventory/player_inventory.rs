use bevy::prelude::*;

use crate::{
    client::state::GameState,
    registry::{
        atlas::GameTextures,
        player_data::{HOTBAR_SIZE, INVENTORY_SIZE},
    },
};

use super::{spawn_player_slot_child, ItemStack};

pub struct PlayerInventoryPlugin;

impl Plugin for PlayerInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InventoryState>()
            .insert_resource(PlayerInventory {
                slots: Vec::with_capacity(INVENTORY_SIZE + HOTBAR_SIZE),
            })
            .add_systems(OnExit(GameState::InGame), reset_state)
            .add_systems(OnEnter(InventoryState::Opened), setup_screen)
            .add_systems(OnExit(InventoryState::Opened), cleanup_screen)
            .add_systems(Update, handle_input.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum InventoryState {
    #[default]
    Closed,
    Opened,
}

#[derive(Component)]
struct PlayerInventoryScreen;

fn reset_state(mut next_state: ResMut<NextState<InventoryState>>) {
    next_state.set(InventoryState::Closed);
}

fn handle_input(
    keyboard: Res<Input<KeyCode>>,
    state: Res<State<InventoryState>>,
    mut next_state: ResMut<NextState<InventoryState>>,
) {
    if keyboard.just_pressed(KeyCode::E) {
        match *state.get() {
            InventoryState::Opened => {
                next_state.set(InventoryState::Closed);
            }
            InventoryState::Closed => {
                next_state.set(InventoryState::Opened);
            }
        }
    }
}

fn setup_screen(mut commands: Commands, window: Query<&Window>, textures: Res<GameTextures>) {
    let window = window.single();
    let screen_size = Vec2::new(window.width() / 2.0, window.height() / 1.75);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(screen_size.x),
                    height: Val::Px(screen_size.y),
                    top: Val::Px((window.height() / 2.0) - (screen_size.y / 2.0)),
                    left: Val::Px((window.width() / 2.0) - (screen_size.x / 2.0)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("Player Inventory"),
            PlayerInventoryScreen,
        ))
        .with_children(|mut commands| {
            for x in 10..INVENTORY_SIZE - 1 {
                spawn_player_slot_child(x, (), &mut commands, &textures);
            }
        });
}

fn cleanup_screen(mut commands: Commands, screens: Query<Entity, With<PlayerInventoryScreen>>) {
    for screen in &screens {
        commands.entity(screen).despawn_recursive();
    }
}

#[derive(Resource)]
pub struct PlayerInventory {
    pub slots: Vec<ItemStack>,
}
