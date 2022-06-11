use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{
    commons::PlayerTokenBuffer,
    events::{WsConsiderRequestingPlayersEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ws_reader: EventReader<WsConsiderRequestingPlayersEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    game_record: Option<Res<GameRecord>>,
    token: Res<PlayerTokenBuffer>,
) {
    if let Some(game_record) = game_record {
        if let Some(ref token) = token.0 {
            for _ in ws_reader.iter() {
                ws_writer.send(WsSendEvent(WsMessage::RequestPlayers(
                    game_record.lobby.clone(),
                    token.clone(),
                )));
            }
        }
    }
}
