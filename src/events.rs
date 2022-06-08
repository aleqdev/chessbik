#[derive(Default)]
pub struct UiLobbyCopyEvent;

#[derive(Default)]
pub struct UiNewGameEvent;

use chessbik_commons::WsMessage;

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WebsocketSendEvent(pub WsMessage);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WebsocketReceiveEvent(pub WsMessage);
);
