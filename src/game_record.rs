use chessbik_board::Board;
use chessbik_commons::{Cell, Lobby, PlayerRecord, PlayersRecord};

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
