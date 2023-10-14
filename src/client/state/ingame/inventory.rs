use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::ui::{AlignSelf, JustifyContent, PositionType, Style, UiRect, Val};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::registry::atlas::GameTextures;
use crate::registry::items::Item;

use self::player_inventory::PlayerInventoryPlugin;

pub mod player_inventory;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerInventoryPlugin)
            .register_type::<PlayerSlot>()
            .register_type::<ItemStack>();
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PlayerSlot(pub usize);

#[derive(Component)]
pub struct SlotItemTexture;

#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub struct ItemStack {
    pub item: Item,
    pub amount: u8,
}

pub fn spawn_player_slot_child(
    index: usize,
    extra: impl Bundle,
    commands: &mut ChildBuilder,
    textures: &GameTextures,
) -> Entity {
    let mut commands = commands.spawn((
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
        Name::new(format!("Player Slot {}", index)),
        PlayerSlot(index as usize),
        extra,
    ));
    commands.with_children(|commands| {
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
    return commands.id();
}
