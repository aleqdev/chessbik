use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{events::{UiRequestEngineEvent, WsSendEvent}, GameRecord, commons::PlayerTokenBuffer};

pub fn system(
    mut ui_reader: EventReader<UiRequestEngineEvent>, 
    mut ws_writer: EventWriter<WsSendEvent>,
    game_record: Option<Res<GameRecord>>,
    token: Res<PlayerTokenBuffer>
) {
    if let Some(game_record) = game_record {
        if let Some(token) = &token.0 {
            for e in ui_reader.iter() {
                ws_writer.send(WsSendEvent(WsMessage::RequestEngineAddition(
                    game_record.lobby.clone(), 
                    e.0, 
                    token.clone()
                )))
            }
        }
    }
}
