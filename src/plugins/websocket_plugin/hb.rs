use bevy::prelude::*;
use chessbik_commons::WsMessage;

use super::convertion::WsSendEvent;

pub fn system(
    mut writer: EventWriter<WsSendEvent>
) {
    writer.send(WsSendEvent(WsMessage::Hb));
}