use bevy::prelude::*;
use crate::jet::Jet;

#[derive(Debug, Component)]
pub struct Camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera_transform = Transform::from_xyz(0.0, 9.5, 35.0).looking_at(Vec3::ZERO, Vec3::Y);

    let camera = (
        Camera3dBundle {
            transform: camera_transform,
            ..default()
        },
        Camera,
    );

    commands.spawn(camera);
}

fn camera_movement(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Jet>)>,
    jet_query: Query<&Transform, (With<Jet>, Without<Camera>)>,
) {
    // for jet_transform in jet_query.into_iter() {
        for mut camera_transform in camera_query.iter_mut() {
            // Get the jet's position
            let jet_transform = jet_query.single();
            let jet_position = jet_transform.translation;

            // Get the camera's position
            let mut camera_position = camera_transform.translation;

            // Set camera's position to follow the jet
            camera_position.x = jet_position.x;
            camera_position.y = jet_position.y + 9.5;
            camera_position.z = jet_position.z + 35.0;

            // Set camera's rotation to look at the jet
            camera_transform.look_at(jet_position, Vec3::Y);

            // Update camera transform
            camera_transform.translation = camera_position;
        }
    // }
}
