use bevy::prelude::*;
use bevy::ui::PositionType;
use bevy::utils::default;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_kira_audio::{AudioChannel, AudioControl};
use rand::Rng;

use crate::client::state::ingame::inventory::ItemStack;
use crate::client::state::GameState;
use crate::registry::atlas::GameTextures;
use crate::registry::audio::{GameSounds, UIChannel};
use crate::registry::items::ItemRegistry;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud)
            .add_systems(OnExit(GameState::InGame), cleanup_hud)
            .add_systems(Update, update_items.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                debug_random_items.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                handle_hover_and_click.run_if(in_state(GameState::InGame)),
            )
            .register_type::<Slot>();
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct Slot {
    item_stack: Option<ItemStack>,
}

#[derive(Component)]
struct ItemTexture;

fn debug_random_items(
    mut slots: Query<&mut Slot>,
    items: Res<ItemRegistry>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::I) {
        info!("Selecting random items...");
        for mut slot in &mut slots {
            slot.item_stack = Some(ItemStack {
                item: items.entities["wall"].clone(),
                amount: rand::thread_rng().gen_range(0..64),
            });
        }
    }
}

fn update_items(
    slots: Query<(&Slot, &Children), Changed<Slot>>,
    mut slot_texture: Query<
        (
            &mut Visibility,
            &mut Handle<TextureAtlas>,
            &mut UiTextureAtlasImage,
        ),
        With<ItemTexture>,
    >,
) {
    for (slot, children) in slots.iter() {
        let (mut visibility, mut slot_texture, mut image) =
            slot_texture.get_mut(children[1]).unwrap();
        match &slot.item_stack {
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
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Slot>)>,
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
                .with_children(|commands| {
                    for x in 0..10 {
                        commands
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        height: Val::Percent(80.0),
                                        margin: UiRect::new(
                                            Val::Px(2.5),
                                            Val::Px(2.5),
                                            Val::Px(2.5),
                                            Val::Px(2.5),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new(format!("Slot {}", x)),
                                Slot { item_stack: None },
                            ))
                            .with_children(|commands| {
                                commands.spawn(AtlasImageBundle {
                                    texture_atlas: textures.ui_inventory.atlas_handle.clone(),
                                    ..default()
                                });
                                commands.spawn((AtlasImageBundle::default(), ItemTexture));
                            });
                    }
                });
        });
}

fn cleanup_hud(mut commands: Commands, roots: Query<Entity, With<Hud>>) {
    for root in &roots {
        commands.entity(root).despawn_recursive();
    }
}
