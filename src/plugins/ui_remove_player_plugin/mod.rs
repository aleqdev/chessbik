use bevy::prelude::*;

use crate::{events::UiRemovePlayerEvent, AppLabels};

mod handle_event;

pub struct RemovePlayerPlugin;

impl Plugin for RemovePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system.label(AppLabels::Ui));
        app.add_event::<UiRemovePlayerEvent>();
    }
}
