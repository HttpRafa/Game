use bevy::app::App;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

pub struct YSortPlugin;

impl Plugin for YSortPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, y_sort).register_type::<YSort>();
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct YSort(pub f32);

fn y_sort(mut entities: Query<(&mut Transform, &YSort)>) {
    for (mut transform, sort) in entities.iter_mut() {
        transform.translation.z =
            sort.0 - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * transform.translation.y))));
    }
}
