use bevy::prelude::*;

use crate::events::MakeMoveEvent;

mod handle_event;

pub struct MakeMovePlugin;

impl Plugin for MakeMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MakeMoveEvent>();
        app.add_system(handle_event::system);
    }
}
