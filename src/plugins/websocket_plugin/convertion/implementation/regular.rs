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
            .send(serde_json::to_string(&**e).expect("failed to serialize event"))
            .expect("failed to convert websocket send event");
    }
}

pub fn receive_system(
    recv_stream: NonSend<WebsocketReceiver>,
    mut recv_events: EventWriter<WebsocketReceiveEvent>,
) {
    use crossbeam_channel::TryRecvError;

    loop {
        let recv = recv_stream.try_recv();
        match recv {
            Ok(str) => recv_events.send(WebsocketReceiveEvent(
                serde_json::from_str(&str).expect("failed to serialize event"),
            )),
            Err(err) => match err {
                TryRecvError::Empty => break,
                TryRecvError::Disconnected => panic!("failed to read from recv_rx"),
            },
        }
    }
}
