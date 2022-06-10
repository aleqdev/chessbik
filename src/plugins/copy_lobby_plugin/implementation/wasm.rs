use bevy::prelude::*;

use crate::{events::UiLobbyCopyEvent, GameRecord};

pub fn system(mut events: EventReader<UiLobbyCopyEvent>, game_record: Option<Res<GameRecord>>) {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(module = "/js/write_clip.js")]
    extern "C" {
        fn write_clip(lobby: String);
    }

    if let Some(game_record) = game_record {
        for _ in events.iter() {
            write_clip(game_record.lobby.to_string());
        }
    }
}
