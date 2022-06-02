pub enum PieceMove {
    Slide(super::PiecePosition),
    Take(super::PiecePosition),
    Castle(),
}
pub trait GetAvailableMoves {
    fn get_available_moves(
        &mut self,
        pos: impl super::IntoPiecePosition,
    ) -> Vec<super::PiecePositionOnArray>;
}

/*impl GetAvailableMoves for super::Field {
    fn get_available_moves(&mut self, pos: impl super::IntoPiecePosition) -> Vec<super::PiecePositionOnArray> {
        let cell = &self.cells[pos.into().as_index()];
        let vec = vec![];
        match **cell {
            None => {},
            Some(piece) => {
                match piece.ty {

                }
            }
        }
        vec
    }
}*/
