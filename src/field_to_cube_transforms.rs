use bevy::prelude::*;

use crate::PiecePosition;

crate::commons::derive_wrapper!(
    #[derive(Clone, Copy)]
    pub struct Field2CubeTransforms([Transform; 54]);
);

impl Default for Field2CubeTransforms {
    fn default() -> Self {
        Self([Default::default(); 54])
    }
}

impl Field2CubeTransforms {
    pub fn register(&mut self, at: impl Into<PiecePosition>, to: Transform) {
        self.0[at.into().as_index()] = to;
    }

    pub fn transform(&self, at: impl Into<PiecePosition>) -> Transform {
        self.0[at.into().as_index()]
    }
}
