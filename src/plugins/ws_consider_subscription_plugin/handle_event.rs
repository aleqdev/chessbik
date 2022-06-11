use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{
    commons::PlayerTokenBuffer,
    events::{WsConsiderSubscriptionEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ws_reader: EventReader<WsConsiderSubscriptionEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    mut commands: Commands,
    token: Res<PlayerTokenBuffer>,
) {
    if let Some(ref token) = token.0 {
        for event in ws_reader.iter() {
            commands.insert_resource(GameRecord::new(event.0.clone()));
            ws_writer.send(WsSendEvent(WsMessage::RequestBoard(event.0.clone())));
            ws_writer.send(WsSendEvent(WsMessage::RequestPlayers(
                event.0.clone(),
                token.clone(),
            )));
            ws_writer.send(WsSendEvent(WsMessage::RequestGameSubscription(
                event.0.clone(),
                token.clone(),
            )));
        }
    }
}
