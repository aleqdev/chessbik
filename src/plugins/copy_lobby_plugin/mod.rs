use bevy::prelude::*;

pub struct CopyLobbyPlugin;

impl Plugin for CopyLobbyPlugin {
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        #[path = "wasm.rs"]
        mod wasm;
        app.add_system(wasm::system);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {
        #[path = "regular.rs"]
        mod regular;
        app.add_system(regular::system);
    }
}
