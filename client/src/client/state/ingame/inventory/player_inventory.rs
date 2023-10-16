use bevy::prelude::*;
use bevy_rapier2d::parry::utils::Array1;

use crate::{
    client::state::{ingame::InGameData, GameState},
    registry::{
        atlas::GameTextures,
        player_data::{HOTBAR_SIZE, INVENTORY_SIZE},
    },
};

use super::{spawn_grid_player_slot_child, ItemStack, Slot, SlotItemTexture, SlotType};

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
            .add_systems(Update, update_items.run_if(in_state(GameState::InGame)))
            .add_systems(Update, handle_input.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct PlayerSlot;

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

fn update_items(
    slots: Query<(&Slot, &Children), With<PlayerSlot>>,
    mut slot_texture: Query<
        (
            &mut Visibility,
            &mut Handle<TextureAtlas>,
            &mut UiTextureAtlasImage,
        ),
        With<SlotItemTexture>,
    >,
    player_inventory: Res<PlayerInventory>,
) {
    if !player_inventory.is_changed() {
        return;
    }
    for (slot, children) in slots.iter() {
        let (mut visibility, mut slot_texture, mut image) =
            slot_texture.get_mut(children[1]).unwrap();
        let item_stack = player_inventory.slots.get_at(slot.index);
        match item_stack {
            Some(item) => {
                let item = &item.item;
                image.index = item.texture_index;
                *slot_texture = item.texture_atlas.atlas_handle.clone_weak();
                *visibility = Visibility::Visible;
            }
            None => {
                *visibility = Visibility::Hidden;
            }
        }
    }
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

fn setup_screen(
    mut commands: Commands,
    textures: Res<GameTextures>,
    mut ingame_data: ResMut<InGameData>,
) {
    ingame_data.screen_open = true;
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            PlayerInventoryScreen,
            Name::new("Player Inventory"),
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            width: Val::Percent(42.5),
                            height: Val::Percent(35.0),
                            grid_template_columns: RepeatedGridTrack::flex(HOTBAR_SIZE as u16, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(
                                (INVENTORY_SIZE / HOTBAR_SIZE) as u16,
                                1.0,
                            ),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Grid"),
                ))
                .with_children(|mut commands| {
                    for x in HOTBAR_SIZE..INVENTORY_SIZE + HOTBAR_SIZE {
                        spawn_grid_player_slot_child(
                            Slot {
                                index: x,
                                slot_type: SlotType::Inventory,
                            },
                            (),
                            &mut commands,
                            &textures,
                        );
                    }
                });
        });
}

fn cleanup_screen(
    mut commands: Commands,
    screens: Query<Entity, With<PlayerInventoryScreen>>,
    mut ingame_data: ResMut<InGameData>,
) {
    for screen in &screens {
        commands.entity(screen).despawn_recursive();
    }
    ingame_data.screen_open = false;
}

#[derive(Resource)]
pub struct PlayerInventory {
    pub slots: Vec<ItemStack>,
}
