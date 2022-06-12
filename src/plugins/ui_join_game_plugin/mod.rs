use bevy::prelude::*;

use crate::events::UiJoinGameEvent;

mod handle_event;

pub struct JoinGamePlugin;

impl Plugin for JoinGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UiJoinGameEvent>();
        app.add_system(handle_event::system);
    }
}
