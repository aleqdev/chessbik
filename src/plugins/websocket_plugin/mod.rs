pub mod receiver;

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

use bevy::prelude::*;

use crate::events::WebsocketEvent;

pub struct WebsocketPlugin;

impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(implementation::system);
        app.add_event::<WebsocketEvent>();
    }
}
