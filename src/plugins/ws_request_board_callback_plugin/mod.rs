use bevy::prelude::*;

use crate::commons::BoardReserve;

mod handle_event;

pub struct RequestBoardCallbackPlugin;

impl Plugin for RequestBoardCallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_event::system);
        app.init_resource::<BoardReserve>();
    }
}
