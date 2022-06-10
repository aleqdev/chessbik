use bevy::{
    ecs::system::NonSend,
    prelude::{EventReader, EventWriter},
};
use chessbik_commons::WsMessage;

use crate::{
    events::{
        WsCreateGameCallbackEvent, WsRequestBoardCallbackEvent, WsRequestPlayerOwningCallbackEvent,
        WsRequestPlayerTokenCallbackEvent, WsSendEvent,
    },
    plugins::websocket_plugin::resources::{WebsocketReceiver, WebsocketSender},
};

pub fn send_system(
    send_stream: NonSend<WebsocketSender>,
    mut send_events: EventReader<WsSendEvent>,
) {
    for e in send_events.iter() {
        send_stream
            .send(serde_json::to_string(&**e).expect("failed to serialize event"))
            .expect("failed to convert websocket send event");
    }
}

pub fn receive_system(
    recv_stream: NonSend<WebsocketReceiver>,
    mut create_game_callback_events: EventWriter<WsCreateGameCallbackEvent>,
    mut request_board_callback_events: EventWriter<WsRequestBoardCallbackEvent>,
    mut request_player_token_callback_events: EventWriter<WsRequestPlayerTokenCallbackEvent>,
    mut request_player_owning_callback_events: EventWriter<WsRequestPlayerOwningCallbackEvent>,
) {
    use crossbeam_channel::TryRecvError;

    loop {
        let recv = recv_stream.try_recv();
        match recv {
            Ok(str) => {
                match serde_json::from_str::<WsMessage>(&str).expect("failed to serialize event") {
                    WsMessage::CreateGameCallback(lobby) => {
                        create_game_callback_events.send(WsCreateGameCallbackEvent(lobby))
                    }
                    WsMessage::RequestBoardCallback(board) => {
                        request_board_callback_events.send(WsRequestBoardCallbackEvent(board))
                    }
                    WsMessage::RequestPlayerTokenCallback(token) => {
                        request_player_token_callback_events
                            .send(WsRequestPlayerTokenCallbackEvent(token))
                    }
                    WsMessage::RequestPlayerOwningCallback(color, owning) => {
                        request_player_owning_callback_events
                            .send(WsRequestPlayerOwningCallbackEvent((color, owning)))
                    }

                    // Server is not panicking because receiving unexpected messages
                    // from potentially untrusted websockets should be ignored.
                    // But to receive unexpected message from server is a controlflow error
                    // and thus to catch it panic should occur.
                    m @ WsMessage::CreateGame
                    | m @ WsMessage::RequestBoard(_)
                    | m @ WsMessage::RequestPlayerToken
                    | m @ WsMessage::RequestOpponentAddition(_, _, _)
                    | m @ WsMessage::RequestEngineAddition(_, _, _)
                    | m @ WsMessage::RequestPlayerOwning(_, _, _)
                    | m @ WsMessage::RequestPlayerRemoval(_, _, _)
                    | m @ WsMessage::JoinGame(_) => panic!("got unexpected ws message{:?}", m),
                }
            }
            Err(err) => match err {
                TryRecvError::Empty => break,
                TryRecvError::Disconnected => panic!("failed to read from recv_rx"),
            },
        }
    }
}
