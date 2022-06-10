use bevy::prelude::*;

mod spawn_lights;

pub struct SidesLightsPlugin;

impl Plugin for SidesLightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_lights::system);
    }
}
