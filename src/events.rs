use chessbik_board::Board;
use chessbik_commons::{Cell, Lobby, PlayerColor, PlayerToken, WsMessage};

#[derive(Default)]
pub struct UiLobbyCopyEvent;

#[derive(Default)]
pub struct UiNewGameEvent;

#[derive(Default)]
pub struct UiLeaveGameEvent;

#[derive(Default)]
pub struct UiJoinGameEvent;

pub struct UiRequestEngineEvent(pub PlayerColor);

pub struct UiRequestOpponentEvent(pub PlayerColor);

chessbik_derive_wrapper::derive_wrapper!(
    #[derive(Default)]
    pub struct UpdateCubeDisplayEvent(pub Vec<usize>);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsSendEvent(pub WsMessage);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsCreateGameCallbackEvent(pub Lobby);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsRequestBoardCallbackEvent(pub Board<Cell>);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsRequestPlayerTokenCallbackEvent(pub PlayerToken);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsRequestPlayerOwningCallbackEvent(pub (PlayerColor, bool));
);
