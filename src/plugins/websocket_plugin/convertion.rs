use bevy::{ecs::system::NonSend, prelude::{EventWriter, EventReader}};

use crate::events::{WebsocketSendEvent, WebsocketReceiveEvent};

use super::resources::{WebsocketReceiver, WebsocketSender};

pub fn send_system(
    send_stream: NonSend<WebsocketSender>,
    mut send_events: EventReader<WebsocketSendEvent>
) {
    for e in send_events.iter() {
        send_stream.send(&e)
            .expect("failed to convert websocket send event");
    }
}

pub fn receive_system(
    recv_stream: NonSend<WebsocketReceiver>,
    mut recv_events: EventWriter<WebsocketReceiveEvent>
) {
    while let Some(str) = recv_stream.recv(None).expect("failed to read from recv_rx") {
        recv_events.send(str.into());
    }
}
