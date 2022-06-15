use bevy::prelude::*;
use bevy_stl::StlPlugin;

pub mod app_assets;
pub mod app_state;
pub mod board_reference;
pub mod commons;
pub mod consts;
pub mod cube;
pub mod cube_transform;
pub mod events;
pub mod game_record;

pub use app_state::*;
pub use board_reference::*;
pub use consts::*;
pub use cube::*;
pub use game_record::*;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::Plugins)
        .add_plugin(StlPlugin)
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 1024.,
            title: "Chessbik".into(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
