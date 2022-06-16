use bevy::prelude::*;

use crate::{commons::BoardReserve, AppLabels};

mod handle_event;

pub struct RequestBoardCallbackPlugin;

impl Plugin for RequestBoardCallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system.label(AppLabels::Websocket).after(AppLabels::Ui));
        app.init_resource::<BoardReserve>();
    }
}
