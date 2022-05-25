#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum PieceFace {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    FORWARD,
    BACK,
}

crate::commons::derive_wrapper!(
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
    pub struct PiecePositionOnFace(pub(crate) usize);
);

crate::commons::derive_wrapper!(
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
    pub struct PiecePositionOnArray(pub(crate) usize);
);

impl PiecePositionOnFace {
    pub fn new(i: impl Into<usize>) -> Self {
        let i = i.into();
        assert!(
            i < 9,
            "PiecePositionOnFace index should be [0; 9), was {}",
            i
        );
        Self(i)
    }
}

impl PiecePositionOnArray {
    pub fn new(i: impl Into<usize>) -> Self {
        let i = i.into();
        assert!(
            i < 54,
            "PiecePositionOnFace index should be [0; 54), was {}",
            i
        );
        Self(i)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum PiecePosition {
    Array(PiecePositionOnArray),
    OnFace(PieceFace, PiecePositionOnFace),
}

impl PiecePosition {
    pub fn array(i: impl Into<usize>) -> Self {
        Self::Array(PiecePositionOnArray::new(i))
    }

    pub fn on_face(face: impl Into<PieceFace>, i: impl Into<usize>) -> Self {
        Self::OnFace(face.into(), PiecePositionOnFace::new(i))
    }

    pub fn as_index(&self) -> usize {
        match self {
            PiecePosition::Array(i) => **i,
            PiecePosition::OnFace(face, face_index) => match face {
                PieceFace::TOP => **face_index,
                PieceFace::BOTTOM => **face_index + 45,
                PieceFace::LEFT => **face_index + 9,
                PieceFace::RIGHT => **face_index + 27,
                PieceFace::FORWARD => **face_index + 18,
                PieceFace::BACK => **face_index + 36,
            },
        }
    }
}

pub trait IntoPiecePosition: Into<super::PiecePosition> {}