use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::AppLabels;

mod configure_visuals;
mod menu_system;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_startup_system(configure_visuals::system);
        app.add_system(menu_system::system.label(AppLabels::Ui));
    }
}
