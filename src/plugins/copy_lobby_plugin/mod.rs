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

use bevy::prelude::*;

use crate::AppLabels;

pub struct CopyLobbyPlugin;

impl Plugin for CopyLobbyPlugin {
    fn build(&self, app: &mut App) {
        use crate::events::UiLobbyCopyEvent;

        app.add_system(implementation::system.label(AppLabels::AfterUi).after(AppLabels::Ui));
        app.add_event::<UiLobbyCopyEvent>();
    }
}
