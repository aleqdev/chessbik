use chessbik_board::PiecePosition;

chessbik_derive_wrapper::derive_wrapper!(
    #[derive(bevy::prelude::Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct BoardReference(pub PiecePosition);
);

impl Default for BoardReference {
    fn default() -> Self {
        Self(PiecePosition(0usize))
    }
}
