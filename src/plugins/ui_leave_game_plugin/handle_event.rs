use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::{
    events::{UiLeaveGameEvent, WsSendEvent},
    GameRecord,
};

pub fn system(
    mut ui_reader: EventReader<UiLeaveGameEvent>,
    mut ws_writer: EventWriter<WsSendEvent>,
    mut commands: Commands,
) {
    for _ in ui_reader.iter() {
        ws_writer.send(WsSendEvent(WsMessage::RequestBoardCallback(
            Default::default(),
        )));
        commands.remove_resource::<GameRecord>();
    }
}
