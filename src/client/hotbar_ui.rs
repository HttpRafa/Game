use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::{AlignItems, BuildChildren, Color, Commands, ImageBundle, NodeBundle, Plugin, Startup, Style, UiRect, Val};
use bevy::ui::PositionType;
use bevy::utils::default;

pub struct HotbarUIPlugin;

impl Plugin for HotbarUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(7.5),
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::BLUE.into(),
        ..default()
    }, Name::new("UI Root"))).with_children(|commands| {
        commands.spawn(ImageBundle {
            ..default()
        });
    });
}