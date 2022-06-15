use chessbik_board::{Board, CubeRotation};
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

    pub fn get_board_after_rotations(&self, rot: CubeRotation) -> Board<Cell> {
        let mut b = self.board.clone();

        let pairs = chessbik_board::cube_rotations_field::get_positions(rot);

        let mut new_b = b.clone();

        for (&from, &to) in pairs[0].iter().zip(pairs[1].iter()) {
            *new_b.at_mut(to) = *b.at(from);
        }

        b = new_b;

        b
    }
}
