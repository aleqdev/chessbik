use bevy::prelude::*;

use crate::events::JoinGameFromClipboardEvent;

mod handle_events;

pub struct JoinGameFromClipboard;

impl Plugin for JoinGameFromClipboard {
    fn build(&self, app: &mut App) {
        app.add_event::<JoinGameFromClipboardEvent>();
        app.add_system(handle_events::system);
    }
}
