use bevy::prelude::*;

use crate::{commons::PlayerTokenBuffer, events::WsRequestPlayerTokenCallbackEvent};

pub fn system(
    mut ws_reader: EventReader<WsRequestPlayerTokenCallbackEvent>,
    mut buffer: ResMut<PlayerTokenBuffer>,
) {
    for event in ws_reader.iter() {
        *buffer = PlayerTokenBuffer(Some(event.0.clone()));
    }
}
