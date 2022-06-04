use bevy::prelude::*;
use bevy_inspector_egui::{self, WorldInspectorPlugin};

pub mod app_materials;
pub mod available_moves_indicator;
pub mod board_reference;
pub mod board_to_cube_transforms;
pub mod commons;
pub mod consts;
pub mod default_cube;

pub use available_moves_indicator::*;
use bevy_stl::StlPlugin;
pub use board_reference::*;
pub use board_to_cube_transforms::*;
pub use consts::*;
use smooth_bevy_cameras::{LookTransformPlugin, controllers::orbit::OrbitCameraPlugin};

mod available_moves_indication_system;
mod initialization_plugin;
mod selection_system;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::Plugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(StlPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::new(true))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 1024.,
            title: "Chessbik".into(),
            ..Default::default()
        })
        .init_resource::<chessbik_board::Board>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(initialization_plugin::system)
        .add_system(selection_system::system)
        .add_system(available_moves_indication_system::system)
        .run();
}
