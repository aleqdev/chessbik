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

pub mod shape_geodesic_field;

use crate::Cell;

pub struct Field {
    pub cells: [Cell; 9 * 6],
}

impl Field {
    pub fn at<'a>(&'a self, fref: impl AsRef<PiecePosition>) -> &'a Cell {
        &self.cells[**fref.as_ref()]
    }

    pub fn translate(&mut self, from: impl AsRef<PiecePosition>, to: impl AsRef<PiecePosition>) {
        self.cells[**to.as_ref()] = self.cells[**from.as_ref()];
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
