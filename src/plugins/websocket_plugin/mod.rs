pub mod connect;
pub mod convertion;
pub mod resources;

use bevy::prelude::*;

use crate::{
    events::{
        WsConsiderRequestingBoardEvent, WsConsiderRequestingPlayersEvent,
        WsConsiderSubscriptionEvent, WsRequestBoardCallbackEvent,
        WsRequestPlayerTokenCallbackEvent, WsRequestPlayersCallbackEvent, WsSendEvent,
    },
    AppLabels,
};

pub struct WebsocketPlugin;

impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(connect::connect_system.exclusive_system());
        app.add_system(
            convertion::send_system
                .label(AppLabels::Websocket)
                .after(AppLabels::Ui),
        );
        app.add_system(
            convertion::receive_system
                .label(AppLabels::Websocket)
                .after(AppLabels::Ui),
        );
        app.add_event::<WsSendEvent>();
        app.add_event::<WsConsiderSubscriptionEvent>();
        app.add_event::<WsRequestBoardCallbackEvent>();
        app.add_event::<WsRequestPlayerTokenCallbackEvent>();
        app.add_event::<WsRequestPlayersCallbackEvent>();
        app.add_event::<WsConsiderRequestingBoardEvent>();
        app.add_event::<WsConsiderRequestingPlayersEvent>();
    }
}
