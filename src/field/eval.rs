crate::commons::derive_wrapper!(
    #[derive(derive_more::Mul, derive_more::Add, derive_more::Sub, derive_more::Div)]
    pub struct Eval(pub f32);
);

pub trait GetEval {
    fn get_eval(&self) -> Eval;
}

impl GetEval for super::Field {
    fn get_eval(&self) -> Eval {
        todo!()
    }
}

impl GetEval for crate::PieceTy {
    fn get_eval(&self) -> Eval {
        match self {
            crate::PieceTy::PAWN => Eval(1.),
            crate::PieceTy::ROOK => Eval(5.),
            crate::PieceTy::KNIGHT => Eval(3.),
            crate::PieceTy::BISHOP => Eval(3.),
            crate::PieceTy::QUEEN => Eval(9.),
            crate::PieceTy::KING => Eval(0.),
            crate::PieceTy::MAGE => Eval(3.),
        }
    }
}

impl GetEval for crate::PieceColor {
    fn get_eval(&self) -> Eval {
        match self {
            crate::PieceColor::WHITE => Eval(1.),
            crate::PieceColor::BLACK => Eval(-1.),
        }
    }
}

impl GetEval for crate::Piece {
    fn get_eval(&self) -> Eval {
        self.ty.get_eval() * *self.color.get_eval()
    }
}
