use bevy::prelude::*;

use crate::events::UiNewGameEvent;

mod handle_event;

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system);
        app.add_event::<UiNewGameEvent>();
    }
}
