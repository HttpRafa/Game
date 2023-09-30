use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::{AlignItems, BuildChildren, Color, Commands, NodeBundle, Plugin, Startup, Style, TextBundle, TextStyle, UiRect, Val};
use bevy::text::Text;
use bevy::utils::default;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::BLUE.into(),
        ..default()
    }, Name::new("UI Root"))).with_children(|commands| {
        commands.spawn(TextBundle {
            text: Text::from_section(
                "Test",
                TextStyle {
                    font_size: 32.0,
                    ..default()
                }
            ),
            ..default()
        });
    });
}