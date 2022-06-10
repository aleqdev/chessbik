use bevy::prelude::*;
use clipboard::{ClipboardContext, ClipboardProvider};

use crate::{events::UiLobbyCopyEvent, GameRecord};

pub fn system(mut events: EventReader<UiLobbyCopyEvent>, game_record: Option<Res<GameRecord>>) {
    if let Some(game_record) = game_record {
        for _ in events.iter() {
            let mut clip: ClipboardContext =
                ClipboardProvider::new().expect("failed to get clipboard");
            clip.set_contents(game_record.lobby.to_string())
                .expect("failed to set clipboard contents");
        }
    }
}
