use bevy::prelude::*;

mod handle_event;

pub struct RequestPlayerTokenCallbackPlugin;

impl Plugin for RequestPlayerTokenCallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system);
    }
}
