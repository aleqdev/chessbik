pub mod material_ty;
pub use material_ty::*;

pub mod cells_materials;
pub use cells_materials::*;

pub mod pieces_materials;
pub use pieces_materials::*;

pub mod impls;
pub use impls::*;

pub struct AppMaterials {
    pub cells: CellsMaterials,
    pub pieces: PiecesMaterials,
    pub selected: MaterialTy,
}
