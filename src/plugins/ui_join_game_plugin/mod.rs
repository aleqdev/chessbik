use bevy::prelude::*;

use crate::events::UiJoinGameEvent;

mod handle_event;
mod initialize_buffer;

pub struct JoinGamePlugin;

impl Plugin for JoinGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UiJoinGameEvent>();
        app.add_startup_system(initialize_buffer::system);
        app.add_system(handle_event::system);
    }
}
