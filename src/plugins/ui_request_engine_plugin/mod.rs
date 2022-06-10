use bevy::prelude::*;

use crate::events::UiRequestEngineEvent;

mod handle_event;

pub struct RequestEnginePlugin;

impl Plugin for RequestEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system);
        app.add_event::<UiRequestEngineEvent>();
    }
}
