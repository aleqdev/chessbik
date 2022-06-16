use bevy::prelude::*;

use crate::{commons::CubeRotationState, AppLabels};

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
        app.add_system(spawn_mage_rotation_indicator::system.label(AppLabels::CubeRotation).after(AppLabels::CubeDisplay));
        app.add_system(update_mage_rotation_indicator::system.label(AppLabels::CubeRotation).after(AppLabels::CubeDisplay));
        app.add_system(update_displayers::system.label(AppLabels::CubeRotation).after(AppLabels::CubeDisplay));
        app.add_system(spawn_rotators::system.label(AppLabels::CubeRotation).after(AppLabels::CubeDisplay));
        app.add_system(update_rotation_state::system.label(AppLabels::CubeRotation).after(AppLabels::CubeDisplay));
        app.add_system(menu::system.label(AppLabels::CubeRotation).after(AppLabels::CubeDisplay));
        app.add_system(update_rotators::system.label(AppLabels::CubeRotation).after(AppLabels::CubeDisplay).after(spawn_rotators::system));
        app.add_system(update_rotators::highlight_system.after(update_rotators::system).after(spawn_rotators::system));
    }
}
