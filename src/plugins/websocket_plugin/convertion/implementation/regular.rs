use bevy::{
    ecs::system::NonSend,
    prelude::{EventReader, EventWriter},
};
use chessbik_commons::WsMessage;

use crate::{
    events::{
        WsConsiderSubscriptionEvent, WsRequestBoardCallbackEvent,
        WsRequestPlayerTokenCallbackEvent, WsConsiderRequestingBoardEvent,
        WsSendEvent, WsConsiderRequestingPlayersEvent, WsRequestPlayersCallbackEvent
    },
    plugins::websocket_plugin::{
        resources::{WebsocketReceiver, WebsocketSender},
    },
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
    mut consider_subscription_events: EventWriter<WsConsiderSubscriptionEvent>,
    mut request_board_callback_events: EventWriter<WsRequestBoardCallbackEvent>,
    mut request_player_token_callback_events: EventWriter<WsRequestPlayerTokenCallbackEvent>,
    mut request_players_callback_events: EventWriter<WsRequestPlayersCallbackEvent>,
    mut consider_requesting_board_events: EventWriter<WsConsiderRequestingBoardEvent>,
    mut consider_requesting_players_events: EventWriter<WsConsiderRequestingPlayersEvent>,
) {
    use crossbeam_channel::TryRecvError;

    loop {
        let recv = recv_stream.try_recv();
        match recv {
            Ok(str) => {
                match serde_json::from_str::<WsMessage>(&str).expect("failed to serialize event") {
                    WsMessage::ConsiderSubscription(lobby) => {
                        consider_subscription_events.send(WsConsiderSubscriptionEvent(lobby))
                    }
                    WsMessage::RequestBoardCallback(board) => {
                        request_board_callback_events.send(WsRequestBoardCallbackEvent(board))
                    }
                    WsMessage::RequestPlayerTokenCallback(token) => {
                        request_player_token_callback_events
                            .send(WsRequestPlayerTokenCallbackEvent(token))
                    }
                    WsMessage::RequestPlayersCallback(players) => {
                        request_players_callback_events.send(WsRequestPlayersCallbackEvent(players))
                    }
                    WsMessage::ConsiderRequestingBoard => {
                        consider_requesting_board_events.send(WsConsiderRequestingBoardEvent)
                    }
                    WsMessage::ConsiderRequestingPlayers => {
                        consider_requesting_players_events.send(WsConsiderRequestingPlayersEvent)
                    }

                    // Server is not panicking because receiving unexpected messages
                    // from potentially untrusted websockets should be ignored.
                    // But to receive unexpected message from server is a controlflow error
                    // and thus to catch it panic should occur.
                    m @ WsMessage::CreateGame
                    | m @ WsMessage::RequestBoard(_)
                    | m @ WsMessage::RequestPlayerToken
                    | m @ WsMessage::RequestOpponentAddition(..)
                    | m @ WsMessage::RequestEngineAddition(..)
                    | m @ WsMessage::RequestPlayerRemoval(..)
                    | m @ WsMessage::RequestPlayers(..)
                    | m @ WsMessage::RequestPlayerNameUpdate(..)
                    | m @ WsMessage::RequestGameSubscription(..)
                    | m @ WsMessage::RequestGameUnsubscription(..)
                    | m @ WsMessage::Hb => {
                        panic!("got unexpected ws message{:?}", m)
                    }
                }
            }
            Err(err) => match err {
                TryRecvError::Empty => break,
                TryRecvError::Disconnected => panic!("failed to read from recv_rx"),
            },
        }
    }
}
