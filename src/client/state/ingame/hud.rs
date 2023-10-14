use crate::client::state::GameState;
use crate::registry::atlas::GameTextures;
use crate::registry::audio::{GameSounds, UIChannel};
use bevy::prelude::*;
use bevy::ui::PositionType;
use bevy::utils::default;
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_rapier2d::parry::utils::Array1;

use super::inventory::player_inventory::PlayerInventory;
use super::inventory::{spawn_player_slot_child, PlayerSlot, SlotItemTexture};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud)
            .add_systems(OnExit(GameState::InGame), cleanup_hud)
            .add_systems(Update, update_items.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                handle_hover_and_click.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HotbarSlot;

fn update_items(
    slots: Query<(&PlayerSlot, &Children)>,
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
        let item_stack = player_inventory.slots.get_at(slot.0);
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

fn handle_hover_and_click(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<HotbarSlot>)>,
    mut hotbar_texture: Query<&mut UiTextureAtlasImage>,
    sounds: Res<GameSounds>,
    audio: Res<AudioChannel<UIChannel>>,
) {
    for (interaction, children) in interaction.iter() {
        let mut hotbar_texture = hotbar_texture.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                hotbar_texture.index = 2;
                audio.play(sounds.ui_click.clone());
            }
            Interaction::Hovered => {
                hotbar_texture.index = 1;
                audio.play(sounds.ui_hover.clone());
            }
            Interaction::None => {
                hotbar_texture.index = 0;
            }
        }
    }
}

fn setup_hud(mut commands: Commands, textures: Res<GameTextures>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    bottom: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(8.0),
                    ..default()
                },
                ..default()
            },
            Hud,
            Name::new("Hud"),
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            height: Val::Percent(100.0),
                            width: Val::Percent(50.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Hotbar Items"),
                ))
                .with_children(|mut commands| {
                    for x in 0..10 {
                        spawn_player_slot_child(x, HotbarSlot, &mut commands, &textures);
                    }
                });
        });
}

fn cleanup_hud(mut commands: Commands, roots: Query<Entity, With<Hud>>) {
    for root in &roots {
        commands.entity(root).despawn_recursive();
    }
}
