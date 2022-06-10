use bevy::prelude::*;

use crate::events::UiLeaveGameEvent;

mod handle_event;

pub struct LeaveGamePlugin;

impl Plugin for LeaveGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UiLeaveGameEvent>();
        app.add_system(handle_event::system);
    }
}
