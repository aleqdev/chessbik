use chessbik_board::Board;
use chessbik_commons::{
    Cell, Lobby, PieceMovePair, PlayerColor, PlayerToken, PlayersRecord, WsMessage,
};

#[derive(Default)]
pub struct UiLobbyCopyEvent;

#[derive(Default)]
pub struct UiNewGameEvent;

#[derive(Default)]
pub struct UiLeaveGameEvent;

#[derive(Default)]
pub struct UiJoinGameEvent;

pub struct JoinGameFromClipboardEvent(pub String);

#[derive(Default)]
pub struct UiChangeNameEvent;

pub struct UiRequestEngineEvent(pub PlayerColor);

pub struct UiRequestOpponentEvent(pub PlayerColor);

pub struct MakeMoveEvent(pub PieceMovePair);

chessbik_derive_wrapper::derive_wrapper!(
    #[derive(Default)]
    pub struct UpdateCubeDisplayEvent(pub Vec<usize>);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsSendEvent(pub WsMessage);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsConsiderSubscriptionEvent(pub Lobby);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsRequestBoardCallbackEvent(pub Board<Cell>);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsRequestPlayerTokenCallbackEvent(pub PlayerToken);
);

chessbik_derive_wrapper::derive_wrapper!(
    pub struct WsRequestPlayersCallbackEvent(pub PlayersRecord);
);

pub struct WsConsiderRequestingBoardEvent;

pub struct WsConsiderRequestingPlayersEvent;
