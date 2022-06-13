use bevy::prelude::*;

use clipboard::{ClipboardContext, ClipboardProvider};

use crate::events::{JoinGameFromClipboardEvent, UiJoinGameEvent};

pub fn startup_system() {}

pub fn read_clip() -> String {
    let mut clip: ClipboardContext = ClipboardProvider::new().expect("failed to get clipboard");

    clip.get_contents()
        .expect("failed to read clipboard contents")
}

pub fn system(
    mut ui_reader: EventReader<UiJoinGameEvent>,
    mut writer: EventWriter<JoinGameFromClipboardEvent>,
) {
    for _ in ui_reader.iter() {
        writer.send(JoinGameFromClipboardEvent(read_clip()));
    }
}
