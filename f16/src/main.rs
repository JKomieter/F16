mod camera;
mod jet;
mod light;
mod land;

use bevy::prelude::*;
use camera::CameraPlugin;
use jet::JetPlugin;
use light::LightPlugin;
use land::LandPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.51, 0.78, 0.90)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(LightPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(JetPlugin)
        .add_plugins(LandPlugin)
        .run();
}
