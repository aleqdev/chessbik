use std::path::PathBuf;

use crate::*;

commons::derive_wrapper!(
    #[derive(Clone, Copy, Component)]
    pub struct Cell {
        pub value: Option<Piece>,
    }
);

impl Cell {
    pub fn get_stl(&self) -> Option<PathBuf> {
        match self.value {
            Some(piece) => Some(match piece.ty {
                PieceTy::PAWN => "pawn.stl".into(),
                PieceTy::ROOK => "rook.stl".into(),
                PieceTy::KNIGHT => "knight.stl".into(),
                PieceTy::BISHOP => "bishop.stl".into(),
                PieceTy::QUEEN => "queen.stl".into(),
                PieceTy::KING => "king.stl".into(),
                PieceTy::MAGE => "mage.stl".into(),
            }),
            None => None,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Some(piece) => piece.fmt(f),
            None => " ".fmt(f),
        }
    }
}

#[allow(non_snake_case)]
pub const fn SomeCell(piece: Piece) -> Cell {
    Cell { value: Some(piece) }
}
#[allow(non_snake_case)]
pub const fn NoneCell() -> Cell {
    Cell { value: None }
}

pub const WPAWN: Cell = SomeCell(Piece {
    ty: PieceTy::PAWN,
    color: PieceColor::WHITE,
});
pub const BPAWN: Cell = SomeCell(Piece {
    ty: PieceTy::PAWN,
    color: PieceColor::BLACK,
});

pub const WKNIGHT: Cell = SomeCell(Piece {
    ty: PieceTy::KNIGHT,
    color: PieceColor::WHITE,
});
pub const BKNIGHT: Cell = SomeCell(Piece {
    ty: PieceTy::KNIGHT,
    color: PieceColor::BLACK,
});

pub const WBISHOP: Cell = SomeCell(Piece {
    ty: PieceTy::BISHOP,
    color: PieceColor::WHITE,
});
pub const BBISHOP: Cell = SomeCell(Piece {
    ty: PieceTy::BISHOP,
    color: PieceColor::BLACK,
});

pub const WROOK: Cell = SomeCell(Piece {
    ty: PieceTy::ROOK,
    color: PieceColor::WHITE,
});
pub const BROOK: Cell = SomeCell(Piece {
    ty: PieceTy::ROOK,
    color: PieceColor::BLACK,
});

pub const WQUEEN: Cell = SomeCell(Piece {
    ty: PieceTy::QUEEN,
    color: PieceColor::WHITE,
});
pub const BQUEEN: Cell = SomeCell(Piece {
    ty: PieceTy::QUEEN,
    color: PieceColor::BLACK,
});

pub const WKING: Cell = SomeCell(Piece {
    ty: PieceTy::KING,
    color: PieceColor::WHITE,
});
pub const BKING: Cell = SomeCell(Piece {
    ty: PieceTy::KING,
    color: PieceColor::BLACK,
});

pub const WMAGE: Cell = SomeCell(Piece {
    ty: PieceTy::MAGE,
    color: PieceColor::WHITE,
});
pub const BMAGE: Cell = SomeCell(Piece {
    ty: PieceTy::MAGE,
    color: PieceColor::BLACK,
});

pub const NONE: Cell = NoneCell();
