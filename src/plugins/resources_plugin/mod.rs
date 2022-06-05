use bevy::prelude::*;

use crate::events::{UiLobbyCopyEvent, UiNewGameEvent};

mod initialize_app_materials;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_app_materials::system);

        app.add_event::<UiLobbyCopyEvent>();
        app.add_event::<UiNewGameEvent>();
    }
}
