use bevy::prelude::*;
use bevy_inspector_egui;

pub mod available_moves_indication_system;
pub mod available_moves_indicator;
pub mod cell;
pub mod commons;
pub mod consts;
pub mod default_cube;
pub mod field;
pub mod field_reference;
pub mod field_to_cube_transforms;
pub mod piece;

pub mod app_materials;

pub use available_moves_indicator::*;
pub use cell::*;
pub use consts::*;
pub use field::*;
pub use field_reference::*;
pub use field_to_cube_transforms::*;
pub use piece::*;

mod initialization_plugin;
mod selection_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<bevy_mod_picking::PickingEvent>()
        .init_resource::<bevy_mod_picking::PickingPluginsState>()
        .add_plugin(bevy_mod_picking::PickingPlugin)
        .add_plugin(bevy_mod_picking::InteractablePickingPlugin)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_stl::StlPlugin)
        .add_plugin(bevy_orbit_controls::OrbitCameraPlugin)
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 1024.,
            title: "Chessbik".into(),
            ..Default::default()
        })
        .init_resource::<field::Field>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(initialization_plugin::system)
        .add_system(selection_system::system)
        .add_system(available_moves_indication_system::system)
        .run();
}
