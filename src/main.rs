use bevy::prelude::*;
use bevy_inspector_egui;

pub mod available_moves_indicator;
pub mod commons;
pub mod consts;
pub mod default_cube;
pub mod board_reference;
pub mod board_to_cube_transforms;
pub mod app_materials;

pub use available_moves_indicator::*;
pub use consts::*;
pub use board_reference::*;
pub use board_to_cube_transforms::*;

mod initialization_plugin;
mod selection_system;
mod available_moves_indication_system;
mod camera_control_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<bevy_mod_picking::PickingEvent>()
        .init_resource::<bevy_mod_picking::PickingPluginsState>()
        .add_plugin(bevy_mod_picking::PickingPlugin)
        .add_plugin(bevy_mod_picking::InteractablePickingPlugin)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_stl::StlPlugin)
        .add_plugin(smooth_bevy_cameras::LookTransformPlugin)
        .add_plugin(smooth_bevy_cameras::controllers::orbit::OrbitCameraPlugin::new(true))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 1024.,
            title: "Chessbik".into(),
            ..Default::default()
        })
        .init_resource::<chessbik_board::Board>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(initialization_plugin::system)
        .add_system(camera_control_system::system)
        .add_system(selection_system::system)
        .add_system(available_moves_indication_system::system)
        .run();
}
