use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::{AtlasImageBundle, BuildChildren, ButtonBundle, Changed, Children, Color, Commands, Component, Interaction, JustifyContent, NodeBundle, Plugin, Query, Res, Startup, Style, UiRect, UiTextureAtlasImage, Update, Val, With};
use bevy::ui::PositionType;
use bevy::utils::default;

use crate::client::textures::GameTextures;

pub struct HotbarUIPlugin;

impl Plugin for HotbarUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, handle_hover_and_click);
    }
}

#[derive(Component)]
struct Slot;

fn handle_hover_and_click(interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Slot>)>, mut hotbar_texture: Query<&mut UiTextureAtlasImage>) {
    for (interaction, children) in interaction.iter() {
        let mut hotbar_texture = hotbar_texture.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Hovered => {
                hotbar_texture.index = 1;
            },
            Interaction::Pressed => {
                hotbar_texture.index = 2;
            },
            Interaction::None => {
                hotbar_texture.index = 0;
            }
        }
    }
}

fn spawn_ui(mut commands: Commands, textures: Res<GameTextures>) {
    commands.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            bottom: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(8.0),
            ..default()
        },
        ..default()
    }, Name::new("UI Root"))).with_children(|commands| {
        commands.spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Percent(100.0),
                width: Val::Percent(50.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }, Name::new("Hotbar Items"))).with_children(|commands| {
            for x in 0..10 {
                commands.spawn((ButtonBundle {
                    style: Style {
                        height: Val::Percent(90.0),
                        margin: UiRect::new(
                            Val::Px(2.5),
                            Val::Px(2.5),
                            Val::Px(2.5),
                            Val::Px(2.5)
                        ),
                        ..default()
                    },
                    background_color: Color::BLUE.into(),
                    ..default()
                }, Name::new(format!("Slot {}", x)), Slot)).with_children(|commands| {
                    commands.spawn(AtlasImageBundle {
                        texture_atlas: textures.ui_inventory.clone(),
                        ..default()
                    });
                });
            }
        });
    });
}