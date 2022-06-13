use bevy::prelude::*;

use crate::events::UiJoinGameEvent;

#[cfg(target_arch = "wasm32")]
mod implementation {
    mod wasm;
    pub use wasm::{startup_system, system};
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    mod regular;
    pub use regular::{startup_system, system};
}

pub use implementation::*;

pub struct JoinGamePlugin;

impl Plugin for JoinGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UiJoinGameEvent>();
        app.add_system(implementation::startup_system);
        app.add_system(implementation::system);
    }
}
