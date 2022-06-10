use chessbik_board::Board;
use chessbik_commons::{Cell, Lobby};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerRecord {
    None,
    Engine,
    Opponent(String, bool),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayersRecord {
    pub white: PlayerRecord,
    pub black: PlayerRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameRecord {
    pub lobby: Lobby,
    pub board: Board<Cell>,
    pub players: PlayersRecord,
}

impl GameRecord {
    pub fn new(lobby: Lobby) -> Self {
        Self {
            lobby,
            board: Default::default(),
            players: PlayersRecord {
                white: PlayerRecord::None,
                black: PlayerRecord::None,
            },
        }
    }
}
