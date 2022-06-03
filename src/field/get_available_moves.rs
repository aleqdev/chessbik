use crate::{shape_geodesic_field, Field, PiecePosition, PieceTy};

pub enum PieceMove {
    Slide(PiecePosition),
    Take(PiecePosition),
    Castle(),
}
pub trait GetAvailableMoves {
    fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PiecePosition>;
}

impl GetAvailableMoves for Field {
    fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PiecePosition> {
        let pos = pos.into();
        let cell = &self.cells[*pos];
        let mut vec = vec![];
        match **cell {
            None => {}
            Some(piece) => match piece.ty {
                PieceTy::PAWN => {}
                PieceTy::ROOK => todo!(),
                PieceTy::KNIGHT => {
                    vec.extend(
                        shape_geodesic_field::KNIGHT_FIELD[*pos]
                            .iter()
                            .map(|&i| PiecePosition(i)),
                    );
                }
                PieceTy::BISHOP => todo!(),
                PieceTy::QUEEN => todo!(),
                PieceTy::KING => todo!(),
                PieceTy::MAGE => todo!(),
            },
        }
        vec
    }
}
