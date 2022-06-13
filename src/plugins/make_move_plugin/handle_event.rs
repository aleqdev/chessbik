use bevy::prelude::*;
use chessbik_board::BoardStatus;
use chessbik_commons::{PlayerColor, WsMessage};

use crate::{commons::PlayerTokenBuffer, events::MakeMoveEvent, events::WsSendEvent, GameRecord};

pub fn system(
    mut reader: EventReader<MakeMoveEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    record: Option<Res<GameRecord>>,
    token: Res<PlayerTokenBuffer>,
) {
    if let Some(record) = record {
        if let Some(ref token) = token.0 {
            for e in reader.iter() {
                ws_writer.send(WsSendEvent(WsMessage::RequestMakeMove(
                    record.lobby.clone(),
                    match record.board.status {
                        BoardStatus::WhitesMove => PlayerColor::WHITE,
                        BoardStatus::BlacksMove => PlayerColor::BLACK,
                        BoardStatus::Mate => return,
                    },
                    token.clone(),
                    e.0.clone(),
                )));
            }
        }
    }
}
