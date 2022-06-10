use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::events::{UiNewGameEvent, WsSendEvent};

pub fn system(mut ui_reader: EventReader<UiNewGameEvent>, mut ws_writer: EventWriter<WsSendEvent>) {
    for _ in ui_reader.iter() {
        ws_writer.send(WsSendEvent(WsMessage::CreateGame))
    }
}
