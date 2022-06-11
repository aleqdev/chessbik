use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{
    events::{WsConsiderRequestingBoardEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ws_reader: EventReader<WsConsiderRequestingBoardEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    game_record: Option<Res<GameRecord>>,
) {
    if let Some(game_record) = game_record {
        for _ in ws_reader.iter() {
            ws_writer.send(WsSendEvent(WsMessage::RequestBoard(
                game_record.lobby.clone(),
            )));
        }
    }
}
