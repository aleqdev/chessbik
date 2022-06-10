pub mod connect;
pub mod convertion;
pub mod resources;

use bevy::prelude::*;

use crate::events::{
    WsCreateGameCallbackEvent, WsRequestBoardCallbackEvent, WsRequestPlayerOwningCallbackEvent,
    WsRequestPlayerTokenCallbackEvent, WsSendEvent,
};

pub struct WebsocketPlugin;

impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(connect::connect_system.exclusive_system());
        app.add_system(convertion::send_system);
        app.add_system(convertion::receive_system);
        app.add_event::<WsSendEvent>();
        app.add_event::<WsCreateGameCallbackEvent>();
        app.add_event::<WsRequestBoardCallbackEvent>();
        app.add_event::<WsRequestPlayerTokenCallbackEvent>();
        app.add_event::<WsRequestPlayerOwningCallbackEvent>();
    }
}
