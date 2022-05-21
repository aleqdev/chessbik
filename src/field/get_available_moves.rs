pub enum PieceMove {
    Slide(super::PiecePosition),
    Take(super::PiecePosition),
    Castle()
}
pub trait GetAvailableMoves {
    fn get_available_moves(&mut self, pos: super::PiecePosition) -> Vec<super::PiecePositionOnArray>;
}

impl GetAvailableMoves for super::Field {
    fn get_available_moves(&mut self, pos: super::PiecePosition) -> Vec<super::PiecePositionOnArray> {
        let cell = &self.cells[pos.as_index()];
        let vec = vec![];
        
        vec
    }
}