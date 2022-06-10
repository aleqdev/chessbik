use bevy::prelude::*;

mod initialize_app_assets;

pub struct AppAssetsPlugin;

impl Plugin for AppAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_app_assets::system);
    }
}
