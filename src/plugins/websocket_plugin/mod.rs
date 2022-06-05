use bevy::prelude::*;

pub struct WebsocketPlugin;

impl Plugin for WebsocketPlugin {
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        use wasm_bindgen::prelude::wasm_bindgen;
        
        #[wasm_bindgen(module = "/js/disable_ctx_menu.js")]
        extern "C" {
            fn disable_ctx_menu();
        }

        app.add_startup_system(|| {
            disable_ctx_menu();
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, _: &mut App) {
        // do nothing
    }
}
