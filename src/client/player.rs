use bevy::app::App;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .register_type::<Player>();
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32
}

fn player_movement(mut characters: Query<(&mut Transform, &Player)>, input: Res<Input<KeyCode>>, time: Res<Time>) {
    let (mut transform, player) = characters.single_mut();
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    if input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement.y += 1.0;
    }
    if input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement.y -= 1.0;
    }
    if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement.x += 1.0;
    }
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement.x -= 1.0;
    }
    transform.translation += movement.normalize_or_zero() * player.speed * time.delta_seconds();
}