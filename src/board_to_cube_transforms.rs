use bevy::prelude::*;

use chessbik_board::PiecePosition;

chessbik_derive_wrapper::derive_wrapper!(
    #[derive(Clone, Copy)]
    pub struct Board2CubeTransforms([Transform; 54]);
);

impl Default for Board2CubeTransforms {
    fn default() -> Self {
        Self([Default::default(); 54])
    }
}

impl Board2CubeTransforms {
    pub fn register(&mut self, at: impl Into<PiecePosition>, to: Transform) {
        self.0[*at.into()] = to;
    }

    pub fn transform(&self, at: impl Into<PiecePosition>) -> Transform {
        self.0[*at.into()]
    }
}
