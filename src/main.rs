use bevy::prelude::*;
use bevy_inspector_egui;

pub mod cell;
pub mod commons;
pub mod consts;
pub mod default_cube;
pub mod field;
pub mod field_reference;
pub mod piece;

pub use cell::*;
pub use consts::*;
pub use field::*;
pub use field_reference::*;
pub use piece::*;

mod initialization_plugin;
mod moving_system;

fn main() {
    println!("{}", std::mem::size_of::<Cell>());
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_stl::StlPlugin)
        .add_plugin(bevy_orbit_controls::OrbitCameraPlugin)
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 1024.,
            title: "Chessbik".into(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(initialization_plugin::system)
        .add_system(moving_system::system)
        .run();
}
