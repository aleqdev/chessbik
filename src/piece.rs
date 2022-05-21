#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceTy {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
    MAGE,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceColor {
    WHITE,
    BLACK,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub ty: PieceTy,
    pub color: PieceColor,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.ty, self.color) {
            (PieceTy::PAWN, PieceColor::WHITE) => "P".fmt(f),
            (PieceTy::ROOK, PieceColor::WHITE) => "R".fmt(f),
            (PieceTy::KNIGHT, PieceColor::WHITE) => "N".fmt(f),
            (PieceTy::BISHOP, PieceColor::WHITE) => "B".fmt(f),
            (PieceTy::QUEEN, PieceColor::WHITE) => "Q".fmt(f),
            (PieceTy::KING, PieceColor::WHITE) => "K".fmt(f),
            (PieceTy::MAGE, PieceColor::WHITE) => "M".fmt(f),

            (PieceTy::PAWN, PieceColor::BLACK) => "p".fmt(f),
            (PieceTy::ROOK, PieceColor::BLACK) => "r".fmt(f),
            (PieceTy::KNIGHT, PieceColor::BLACK) => "n".fmt(f),
            (PieceTy::BISHOP, PieceColor::BLACK) => "b".fmt(f),
            (PieceTy::QUEEN, PieceColor::BLACK) => "q".fmt(f),
            (PieceTy::KING, PieceColor::BLACK) => "k".fmt(f),
            (PieceTy::MAGE, PieceColor::BLACK) => "n".fmt(f),
        }
    }
}
