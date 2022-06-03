
use std::slice::SliceIndex;

use crate::{shape_geodesic_field, Field, PiecePosition, PieceTy, PieceMove, PieceColor};


pub trait GetAvailableMoves {
    fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PieceMove>;
}


impl GetAvailableMoves for Field {
    fn get_available_moves(&self, pos: impl Into<PiecePosition>) -> Vec<PieceMove> {
        let pos = pos.into();
        let cell = &self.cells[*pos];
        let mut vec = vec![];
        match **cell {
            None => {}
            Some(piece) => match piece.ty {
                PieceTy::PAWN => {}
                PieceTy::ROOK => vec.extend(geodesic_calculator(
                    pos,
                    piece.color,
                    ..,
                    ..0,
                    self
                )),
                PieceTy::KNIGHT => {
                    vec.extend(
                        shape_geodesic_field::KNIGHT_FIELD[*pos]
                            .iter()
                            .filter_map(|&i| {
                                match self.at(i).value {
                                    Some(at) => if at.color != piece.color {
                                        Some(PieceMove::take(i))
                                    } else {
                                        None
                                    },
                                    None => Some(PieceMove::slide(i))
                                }
                            })
                    );
                }
                PieceTy::BISHOP => vec.extend(geodesic_calculator(
                    pos,
                    piece.color,
                    ..0,
                    ..,
                    self
                )),
                PieceTy::QUEEN => vec.extend(geodesic_calculator(
                    pos,
                    piece.color,
                    ..,
                    ..,
                    self
                )),
                PieceTy::KING => vec.extend(geodesic_calculator(
                    pos,
                    piece.color,
                    ..1,
                    ..1,
                    self
                )),
                PieceTy::MAGE => vec.extend(geodesic_calculator(
                    pos,
                    piece.color,
                    ..1,
                    ..1,
                    self
                )),
            },
        }
        vec
    }
}

fn geodesic_calculator<T, U>(
    pos: PiecePosition,
    color: PieceColor, 
    range_per: T, 
    range_diag: U,
    field: &Field
) -> Vec<PieceMove>
where 
    T: SliceIndex<[usize], Output=[usize]> + Copy,
    U: SliceIndex<[usize], Output=[usize]> + Copy
{
    let mut v = vec!();

    for &per in shape_geodesic_field::PERPENDICULAR_FIELD[*pos] {
        for &i in per[range_per].iter() {
            if let Some(piece) = field.at(i).value {
                if piece.color != color {
                    v.push(PieceMove::take(i));
                }
                break;
            }
            v.push(PieceMove::slide(i))
        }
    }

    for &diag in shape_geodesic_field::DIAGONAL_FIELD[*pos] {
        for &i in diag[range_diag].iter() {
            if let Some(piece) = field.at(i).value {
                if piece.color != color {
                    v.push(PieceMove::take(i));
                }
                break;
            }
            v.push(PieceMove::slide(i))
        }
    }

    v
}