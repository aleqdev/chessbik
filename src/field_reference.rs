use crate::PiecePosition;

crate::commons::derive_wrapper!(
    #[derive(bevy::prelude::Component, Clone, Copy)]
    pub struct FieldReference(PiecePosition);
);

impl Default for FieldReference {
    fn default() -> Self {
        Self(PiecePosition(0usize))
    }
}
