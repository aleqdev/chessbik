use bevy::prelude::*;

use crate::events::UiRequestOpponentEvent;

mod handle_event;

pub struct RequestOpponentPlugin;

impl Plugin for RequestOpponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system);
        app.add_event::<UiRequestOpponentEvent>();
    }
}
