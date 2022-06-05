use bevy::prelude::*;
use clipboard::{ClipboardContext, ClipboardProvider};

use crate::events::UiLobbyCopyEvent;

pub fn system(
    mut events: EventReader<UiLobbyCopyEvent>
) {
    for _ in events.iter() {
        let mut clip: ClipboardContext = ClipboardProvider::new().expect("failed to get clipboard");
        clip.set_contents("123123".to_string()).expect("failed to set clipboard contents");
    }
}