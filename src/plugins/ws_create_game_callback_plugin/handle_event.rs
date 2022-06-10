use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{
    events::{WsCreateGameCallbackEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ws_reader: EventReader<WsCreateGameCallbackEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    mut commands: Commands,
) {
    for event in ws_reader.iter() {
        commands.insert_resource(GameRecord::new(event.0.clone()));
        ws_writer.send(WsSendEvent(WsMessage::RequestBoard(event.0.clone())));
    }
}
