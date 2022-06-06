pub mod resources;
pub mod convertion;
mod simple;


#[cfg(target_arch = "wasm32")]
mod implementation {
    use super::*;
    mod wasm;
    pub use wasm::system;
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    use super::*;
    mod regular;
    pub use regular::system;
}

use bevy::{prelude::*, core::FixedTimestep};

use crate::events::{WebsocketSendEvent, WebsocketReceiveEvent};

pub struct WebsocketPlugin;

impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(implementation::system.exclusive_system());
        app.add_system(convertion::send_system);
        app.add_system(convertion::receive_system);
        app.add_event::<WebsocketSendEvent>();
        app.add_event::<WebsocketReceiveEvent>();
        app.add_startup_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.))
                .with_system(simple::read_system)
                .with_system(simple::write_system)
        );
    }
}
