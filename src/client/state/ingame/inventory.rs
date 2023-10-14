use bevy::prelude::*;
use bevy::ui::{AlignSelf, JustifyContent, PositionType, Style, UiRect, Val};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::client::state::GameState;
use crate::registry::atlas::GameTextures;
use crate::registry::audio::{GameSounds, UIChannel};
use crate::registry::items::Item;

use self::player_inventory::{PlayerInventoryPlugin, PlayerSlot};

pub mod player_inventory;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerInventoryPlugin)
            .add_systems(
                Update,
                handle_slot_hover_and_click.run_if(in_state(GameState::InGame)),
            )
            .register_type::<SlotType>()
            .register_type::<Slot>()
            .register_type::<ItemStack>();
    }
}

#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub enum SlotType {
    #[default]
    Hotbar,
    Inventory,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Slot {
    pub index: usize,
    pub slot_type: SlotType,
}

#[derive(Component)]
pub struct SlotItemTexture;

#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct ItemStack {
    pub item: Item,
    pub amount: u8,
}

fn handle_slot_hover_and_click(
    interaction: Query<(&Interaction, &Children, &Slot), Changed<Interaction>>,
    mut hotbar_texture: Query<&mut UiTextureAtlasImage>,
    sounds: Res<GameSounds>,
    audio: Res<AudioChannel<UIChannel>>,
) {
    for (interaction, children, slot) in interaction.iter() {
        let mut hotbar_texture = hotbar_texture.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                hotbar_texture.index = 2;
                audio.play(sounds.ui_click.clone());
            }
            Interaction::Hovered => {
                hotbar_texture.index = 1;
                match slot.slot_type {
                    SlotType::Hotbar => {
                        audio.play(sounds.ui_hover.clone());
                    }
                    SlotType::Inventory => {}
                }
            }
            Interaction::None => {
                hotbar_texture.index = 0;
            }
        }
    }
}

pub fn spawn_grid_player_slot_child(
    slot: Slot,
    extra: impl Bundle,
    commands: &mut ChildBuilder,
    textures: &GameTextures,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Name::new(format!("Grid Slot {}", slot.index)),
        ))
        .with_children(|mut commands| {
            spawn_player_slot_child(slot, extra, &mut commands, &textures);
        });
}

pub fn spawn_player_slot_child(
    slot: Slot,
    extra: impl Bundle,
    commands: &mut ChildBuilder,
    textures: &GameTextures,
) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    height: Val::Percent(80.0),
                    margin: UiRect::new(Val::Px(2.5), Val::Px(2.5), Val::Px(2.5), Val::Px(2.5)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            Name::new(format!("Player Slot {}", slot.index)),
            PlayerSlot,
            slot,
            extra,
        ))
        .with_children(|commands| {
            commands.spawn(AtlasImageBundle {
                texture_atlas: textures.ui_inventory.atlas_handle.clone(),
                ..default()
            });
            commands.spawn((
                AtlasImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                },
                SlotItemTexture,
            ));
        });
}
