use crate::client::state::GameState;
use crate::registry::atlas::GameTextures;
use bevy::prelude::*;
use bevy::ui::PositionType;
use bevy::utils::default;

use super::inventory::{spawn_player_slot_child, Slot, SlotType};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud)
            .add_systems(OnExit(GameState::InGame), cleanup_hud)
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

fn handle_hover_and_click(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<HotbarSlot>)>,
) {
    for (interaction, _children) in interaction.iter() {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {}
            Interaction::None => {}
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
                        spawn_player_slot_child(
                            Slot {
                                index: x as usize,
                                slot_type: SlotType::Hotbar,
                            },
                            HotbarSlot,
                            &mut commands,
                            &textures,
                        );
                    }
                });
        });
}

fn cleanup_hud(mut commands: Commands, roots: Query<Entity, With<Hud>>) {
    for root in &roots {
        commands.entity(root).despawn_recursive();
    }
}
