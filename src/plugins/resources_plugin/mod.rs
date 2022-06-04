use bevy::prelude::*;

mod initialize_app_materials;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_app_materials::system);
    }
}