#[derive(Default)]
pub struct UiLobbyCopyEvent;

#[derive(Default)]
pub struct UiNewGameEvent;

chessbik_commons::derive_wrapper!(
    #[derive(Default)]
    pub struct WebsocketSendEvent(pub String);
);

chessbik_commons::derive_wrapper!(
    #[derive(Default)]
    pub struct WebsocketReceiveEvent(pub String);
);
