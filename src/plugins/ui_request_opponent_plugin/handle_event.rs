use bevy::prelude::*;
use chessbik_commons::{OpponentName, WsMessage};

use crate::{
    commons::{PlayerNameBuffer, PlayerTokenBuffer},
    events::{UiRequestOpponentEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ui_reader: EventReader<UiRequestOpponentEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    game_record: Option<Res<GameRecord>>,
    token: Res<PlayerTokenBuffer>,
    name: Res<PlayerNameBuffer>,
) {
    if let Some(game_record) = game_record {
        if let Some(token) = &token.0 {
            for e in ui_reader.iter() {
                ws_writer.send(WsSendEvent(WsMessage::RequestOpponentAddition(
                    game_record.lobby.clone(),
                    e.0,
                    token.clone(),
                    OpponentName(name.0.clone()),
                )))
            }
        }
    }
}
