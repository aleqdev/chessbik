use crate::PiecePosition;

pub enum PieceMoveTy {
    Slide,
    Take,
    Castle
}

pub struct PieceMove {
    pub pos: PiecePosition,
    pub ty: PieceMoveTy
}

impl PieceMove {
    pub fn slide(pos: impl Into<PiecePosition>) -> Self {
        Self {
            pos: pos.into(),
            ty: PieceMoveTy::Slide
        }
    }

    pub fn take(pos: impl Into<PiecePosition>) -> Self {
        Self {
            pos: pos.into(),
            ty: PieceMoveTy::Take
        }
    }

    pub fn castle(pos: impl Into<PiecePosition>) -> Self {
        Self {
            pos: pos.into(),
            ty: PieceMoveTy::Castle
        }
    }
}