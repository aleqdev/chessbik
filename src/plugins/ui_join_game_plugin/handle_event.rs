use bevy::prelude::*;
use chessbik_commons::{Lobby, WsMessage};

use crate::{
    commons::PlayerTokenBuffer,
    events::{UiJoinGameEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ui_reader: EventReader<UiJoinGameEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    mut commands: Commands,
    token: Res<PlayerTokenBuffer>,
) {
    if let Some(ref token) = token.0 {
        for e in ui_reader.iter() {
            let lobby = Lobby::new(e.0.clone());
            commands.insert_resource(GameRecord::new(Lobby::new(lobby.clone())));
            ws_writer.send(WsSendEvent(WsMessage::RequestBoard(lobby.clone())));
            ws_writer.send(WsSendEvent(WsMessage::RequestPlayers(
                lobby.clone(),
                token.clone(),
            )));
            ws_writer.send(WsSendEvent(WsMessage::RequestGameSubscription(
                lobby,
                token.clone(),
            )));
        }
    }
}
