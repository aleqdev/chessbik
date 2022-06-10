use bevy::{
    ecs::system::NonSend,
    prelude::{EventReader, EventWriter},
};
use chessbik_commons::WsMessage;

use crate::{
    events::{WsCreateGameCallbackEvent, WsRequestBoardCallbackEvent, WsSendEvent},
    plugins::websocket_plugin::resources::{WebsocketReceiver, WebsocketSender},
};

pub fn send_system(
    send_stream: NonSend<WebsocketSender>,
    mut send_events: EventReader<WsSendEvent>,
) {
    for e in send_events.iter() {
        send_stream
            .send(&serde_json::to_string(&**e).expect("failed to serialize event"))
            .expect("failed to convert websocket send event");
    }
}

pub fn receive_system(
    recv_stream: NonSend<WebsocketReceiver>,
    mut create_game_callback_events: EventWriter<WsCreateGameCallbackEvent>,
    mut request_board_callback_events: EventWriter<WsRequestBoardCallbackEvent>,
) {
    while let Some(string) = recv_stream.recv(None).expect("failed to read from recv_rx") {
        match serde_json::from_str::<WsMessage>(&string).expect("failed to serialize event") {
            WsMessage::CreateGameCallback(lobby) => {
                create_game_callback_events.send(WsCreateGameCallbackEvent(lobby))
            }
            WsMessage::RequestBoardCallback(board) => {
                request_board_callback_events.send(WsRequestBoardCallbackEvent(board))
            }
            // Server is not panicking because receiving unexpected messages
            // from potentially untrusted websockets should be ignored.
            // But to receive unexpected message from server is a controlflow error
            // and thus to catch it panic should occur.
            _ => panic!("got unexpected ws message"),
        }
    }
}
