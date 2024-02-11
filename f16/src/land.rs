use bevy::prelude::*;

pub struct LandPlugin;

impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_land);
    }
}

fn spawn_land(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let land_transform = Transform::from_xyz(0.0, -20.0, 0.0).with_scale(Vec3::splat(4.1));

    // let land = (
    //     SceneBundle {
    //         scene: asset_server.load("Low-poly landscape.glb#Scene0"),
    //         transform: land_transform,
    //         ..Default::default()
    //     },
    // );

    let land = PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: land_transform,
        ..default()
    };

    commands.spawn(land);
}
