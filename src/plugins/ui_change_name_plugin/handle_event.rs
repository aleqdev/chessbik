use bevy::prelude::*;
use chessbik_commons::{IsOwning, OpponentName, PlayerColor, PlayerRecord, WsMessage};

use crate::{
    commons::{PlayerNameBuffer, PlayerTokenBuffer},
    events::{UiChangeNameEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ui_reader: EventReader<UiChangeNameEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    game_record: Option<Res<GameRecord>>,
    token: Res<PlayerTokenBuffer>,
    name: Res<PlayerNameBuffer>,
) {
    if let Some(game_record) = game_record {
        if let Some(token) = &token.0 {
            for _ in ui_reader.iter() {
                if let PlayerRecord::Opponent(_, IsOwning(true)) = game_record.players.white {
                    ws_writer.send(WsSendEvent(WsMessage::RequestPlayerNameUpdate(
                        game_record.lobby.clone(),
                        PlayerColor::WHITE,
                        token.clone(),
                        OpponentName(name.0.clone()),
                    )))
                }

                if let PlayerRecord::Opponent(_, IsOwning(true)) = game_record.players.black {
                    ws_writer.send(WsSendEvent(WsMessage::RequestPlayerNameUpdate(
                        game_record.lobby.clone(),
                        PlayerColor::BLACK,
                        token.clone(),
                        OpponentName(name.0.clone()),
                    )))
                }
            }
        }
    }
}
