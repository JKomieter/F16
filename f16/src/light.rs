use bevy::prelude::*;


pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(-0.4, 2.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            color: Color::rgb(0.99, 0.98, 0.83),
            shadows_enabled: true,
            illuminance: 0.8,
            ..default()
        },
        ..default()
    });
}