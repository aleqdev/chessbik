pub mod connect;
pub mod convertion;
pub mod resources;

use bevy::prelude::*;

pub use crate::events::{WebsocketReceiveEvent, WebsocketSendEvent};

pub struct WebsocketPlugin;

impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(connect::connect_system.exclusive_system());
        app.add_system(convertion::send_system);
        app.add_system(convertion::receive_system);
        app.add_event::<WebsocketSendEvent>();
        app.add_event::<WebsocketReceiveEvent>();
    }
}
