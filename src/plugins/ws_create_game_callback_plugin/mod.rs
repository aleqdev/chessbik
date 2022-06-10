use bevy::prelude::*;

mod handle_event;

pub struct CreateGameCallbackPlugin;

impl Plugin for CreateGameCallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system);
    }
}
