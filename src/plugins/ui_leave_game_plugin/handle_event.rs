use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{
    commons::PlayerTokenBuffer,
    events::{UiLeaveGameEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ui_reader: EventReader<UiLeaveGameEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    mut commands: Commands,
    game_record: Option<Res<GameRecord>>,
    token: Res<PlayerTokenBuffer>,
) {
    if let Some(game_record) = game_record {
        if let Some(ref token) = token.0 {
            for _ in ui_reader.iter() {
                ws_writer.send(WsSendEvent(WsMessage::RequestGameUnsubscription(
                    game_record.lobby.clone(),
                    token.clone(),
                )));
                commands.remove_resource::<GameRecord>();
            }
        }
    }
}
