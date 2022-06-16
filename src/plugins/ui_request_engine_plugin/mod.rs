use bevy::prelude::*;

use crate::{events::UiRequestEngineEvent, AppLabels};

mod handle_event;

pub struct RequestEnginePlugin;

impl Plugin for RequestEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system.label(AppLabels::Ui));
        app.add_event::<UiRequestEngineEvent>();
    }
}
