use bevy::prelude::*;

use crate::events::UiLobbyCopyEvent;

pub fn system(
    mut events: EventReader<UiLobbyCopyEvent>
) {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(module = "/js/write_clip.js")]
    extern "C" {
        fn write_clip(lobby: String);
    }

    for _ in events.iter() {
        write_clip("123".to_string());
    }
}