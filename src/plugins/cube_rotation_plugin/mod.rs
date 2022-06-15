use bevy::prelude::*;

use crate::commons::CubeRotationState;

mod menu;
mod spawn_mage_rotation_indicator;
mod spawn_rotators;
mod update_displayers;
mod update_mage_rotation_indicator;
mod update_rotation_state;
mod update_rotators;

pub struct CubeRotationPlugin;

impl Plugin for CubeRotationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CubeRotationState>();
        app.add_system(spawn_mage_rotation_indicator::system);
        app.add_system(update_mage_rotation_indicator::system);
        app.add_system(update_displayers::system);
        app.add_system(spawn_rotators::system);
        app.add_system(update_rotation_state::system);
        app.add_system(menu::system);
        app.add_system(update_rotators::system);
    }
}
