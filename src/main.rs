use bevy::prelude::*;
use bevy_stl::StlPlugin;

pub mod app_assets;
pub mod app_labels;
pub mod board_reference;
pub mod commons;
pub mod consts;
pub mod cube;
pub mod cube_transform;
pub mod events;
pub mod game_record;

pub use app_labels::*;
pub use board_reference::*;
pub use consts::*;
pub use cube::*;
pub use game_record::*;

mod plugins;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(plugins::Plugins)
        .add_plugin(StlPlugin)
        .insert_resource(ClearColor(Color::BLACK));

    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();

        app.insert_resource(WindowDescriptor {
            width: window.inner_width().unwrap().as_f64().unwrap() as f32 - 12.,
            height: window.inner_height().unwrap().as_f64().unwrap() as f32 - 12.,
            title: "Chessbik".into(),
            ..Default::default()
        });
    }

    app.run();
}
