pub fn get_piece_stl(ty: crate::PieceTy) -> std::path::PathBuf {
    match ty {
        crate::PieceTy::PAWN => "pawn.stl".into(),
        crate::PieceTy::ROOK => "rook.stl".into(),
        crate::PieceTy::KNIGHT => "knight.stl".into(),
        crate::PieceTy::BISHOP => "bishop.stl".into(),
        crate::PieceTy::QUEEN => "queen.stl".into(),
        crate::PieceTy::KING => "king.stl".into(),
        crate::PieceTy::MAGE => "mage.stl".into(),
    }
}
