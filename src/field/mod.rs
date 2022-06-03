pub mod field_new;
pub use field_new::*;

pub mod field_print_debug;
pub use field_print_debug::*;

pub mod eval;
pub use eval::*;

pub mod piece_position;
pub use piece_position::*;

pub mod get_available_moves;
pub use get_available_moves::*;

pub mod piece_move;
pub use piece_move::*;

pub mod shape_geodesic_field;

use crate::Cell;

pub struct Field {
    pub cells: [Cell; 9 * 6],
}

impl Field {
    pub fn at<'a>(&'a self, fref: impl Into<PiecePosition>) -> &'a Cell {
        &self.cells[*fref.into()]
    }

    pub fn translate(&mut self, from: impl Into<PiecePosition>, to: impl Into<PiecePosition>) {
        self.cells[*to.into()] = self.cells[*from.into()];
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
