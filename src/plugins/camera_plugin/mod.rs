use bevy::prelude::*;
use smooth_bevy_cameras::{LookTransformPlugin, controllers::orbit::OrbitCameraPlugin};

mod override_input_system;
mod spawn_system;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LookTransformPlugin);
        app.add_plugin(OrbitCameraPlugin::new(true));
        app.add_system(override_input_system::system);
        app.add_startup_system(spawn_system::system);
    }
}
