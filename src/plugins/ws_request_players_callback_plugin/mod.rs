use bevy::prelude::*;

use crate::AppLabels;

mod handle_event;

pub struct RequestPlayersCallbackPlugin;

impl Plugin for RequestPlayersCallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system.label(AppLabels::Websocket).after(AppLabels::Ui));
    }
}
