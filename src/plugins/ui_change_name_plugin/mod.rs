use bevy::prelude::*;

use crate::{events::UiChangeNameEvent, AppLabels};

mod handle_event;

pub struct ChangeNamePlugin;

impl Plugin for ChangeNamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system.label(AppLabels::Ui));
        app.add_event::<UiChangeNameEvent>();
    }
}
