use bevy::{
    ecs::system::NonSend,
    prelude::{EventReader, EventWriter},
};

use crate::{
    events::{WebsocketReceiveEvent, WebsocketSendEvent},
    plugins::websocket_plugin::resources::{WebsocketReceiver, WebsocketSender},
};

pub fn send_system(
    send_stream: NonSend<WebsocketSender>,
    mut send_events: EventReader<WebsocketSendEvent>,
) {
    for e in send_events.iter() {
        send_stream
            .send(&serde_json::to_string(&**e).expect("failed to serialize event"))
            .expect("failed to convert websocket send event");
    }
}

pub fn receive_system(
    recv_stream: NonSend<WebsocketReceiver>,
    mut recv_events: EventWriter<WebsocketReceiveEvent>,
) {
    while let Some(string) = recv_stream.recv(None).expect("failed to read from recv_rx") {
        recv_events.send(WebsocketReceiveEvent(
            serde_json::from_str(&string).expect("failed to serialize event"),
        ));
    }
}
