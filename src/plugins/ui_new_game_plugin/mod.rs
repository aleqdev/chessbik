use bevy::prelude::*;

use crate::{events::UiNewGameEvent, AppLabels};

mod handle_event;

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system.label(AppLabels::Ui));
        app.add_event::<UiNewGameEvent>();
    }
}
