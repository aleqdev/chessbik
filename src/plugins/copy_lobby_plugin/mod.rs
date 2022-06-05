use bevy::prelude::*;


#[cfg(target_arch = "wasm32")]
mod implementation {
    mod wasm;
    pub use wasm::system;
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    mod regular;
    pub use regular::system;
}

pub struct CopyLobbyPlugin;

impl Plugin for CopyLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(implementation::system);
    }
}
