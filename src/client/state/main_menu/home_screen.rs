use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::{AlignItems, BackgroundColor, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component, DespawnRecursiveExt, Entity, in_state, Interaction, IntoSystemConfigs, JustifyContent, NextState, NodeBundle, OnEnter, OnExit, Plugin, Query, ResMut, Style, TextBundle, TextStyle, Update, Val, With};
use bevy::utils::default;

use crate::client::state::GameState;
use crate::client::state::main_menu::MainMenuState;

pub struct HomeScreenPlugin;

impl Plugin for HomeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::HomeScreen), setup_menu)
            .add_systems(OnExit(MainMenuState::HomeScreen), cleanup_menu)
            .add_systems(Update, handle_interaction.run_if(in_state(MainMenuState::HomeScreen)));
    }
}

#[derive(Component)]
struct HomeScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn handle_interaction(mut game_state: ResMut<NextState<GameState>>, mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                game_state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }, HomeScreen, Name::new("Home Screen"))).with_children(|commands| {
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(400.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.258, 0.271, 0.286).into(),
            ..default()
        }).with_children(|commands| {
            commands.spawn((ButtonBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            }, Name::new("Singleplayer Button"))).with_children(|commands| {
                commands.spawn(TextBundle::from_section("Singleplayer", TextStyle {
                    font_size: 17.5,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                }));
            });
        });
    });
}

fn cleanup_menu(mut commands: Commands, roots: Query<Entity, With<HomeScreen>>) {
    for root in &roots {
        commands.entity(root).despawn_recursive();
    }
}