use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::{
    client::state::GameState,
    registry::{
        items::Item,
        player_data::{HOTBAR_SIZE, INVENTORY_SIZE},
    },
};

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
            .add_systems(Update, open_inventory.run_if(in_state(GameState::InGame)));
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

fn open_inventory(
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

fn setup_screen(mut _commands: Commands) {
    info!("Setup player inventory..");
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

#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct ItemStack {
    pub item: Item,
    pub amount: u8,
}
