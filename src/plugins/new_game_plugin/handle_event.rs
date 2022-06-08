use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{events::UiNewGameEvent, plugins::websocket_plugin::WebsocketSendEvent};

pub fn system(
    mut ui_reader: EventReader<UiNewGameEvent>,
    mut ws_writer: EventWriter<WebsocketSendEvent>,
) {
    for _ in ui_reader.iter() {
        ws_writer.send(WebsocketSendEvent(WsMessage::CreateGame))
    }
}
