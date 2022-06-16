use bevy::prelude::*;

use crate::{events::UiRequestOpponentEvent, AppLabels};

mod handle_event;

pub struct RequestOpponentPlugin;

impl Plugin for RequestOpponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system.label(AppLabels::Ui));
        app.add_event::<UiRequestOpponentEvent>();
    }
}
