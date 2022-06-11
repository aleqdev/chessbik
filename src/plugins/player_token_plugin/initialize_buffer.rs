use bevy::prelude::*;
use chessbik_commons::WsMessage;

use crate::commons::PlayerTokenBuffer;
use crate::events::WsSendEvent;

pub fn system(mut commands: Commands, mut request_writer: EventWriter<WsSendEvent>) {
    commands.insert_resource(PlayerTokenBuffer(None));
    request_writer.send(WsSendEvent(WsMessage::RequestPlayerToken));
}
