use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Jet;

#[allow(dead_code)]
#[derive(Debug, Component)]
pub struct JetMovement {
    roll_speed: f32,
    velocity: f32,
    acceleration: f32,
    low_speed: f32,
    high_speed: f32,
    transform: Transform,
}

impl Default for JetMovement {
    fn default() -> Self {
        JetMovement {
            roll_speed: 1.0,
            velocity: 1.9,
            acceleration: 2.4,
            low_speed: 1.0,
            high_speed: 3.0,
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
        }
    }
}

pub struct JetPlugin;

impl Plugin for JetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_jet)
            .add_systems(Update, jet_movement);
    }
}

fn spawn_jet(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut jet_transform = Transform::from_xyz(0.0, 0.0, 5.0);
    jet_transform.rotate_local_y(-91.2);
    let jet = (
        SceneBundle {
            scene: asset_server.load("Jet.glb#Scene0"),
            transform: jet_transform,
            ..default()
        },
        Jet,
        JetMovement::default(),
    );

    commands.spawn(jet);
}

fn jet_movement(
    mut jet_query: Query<(&mut Transform, &JetMovement), With<Jet>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut transform, movement) in jet_query.iter_mut() {
        let mut direction = 0.0;
        let mut rotation = 0.3;

        if keyboard_input.pressed(KeyCode::Right) {
            direction += movement.velocity;
            rotation -= movement.roll_speed;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= movement.velocity;
        }

        // direct the jet left or right and tilt its local z axis a little
        transform.rotate(Quat::from_rotation_z(rotation * time.delta_seconds()));
        transform.rotate(Quat::from_rotation_y(direction * time.delta_seconds()));
        
    }
}
