pub use super::MaterialTy;

pub mod materials_for_sides;
pub use materials_for_sides::*;

pub struct CellsMaterials {
    pub default: MaterialsForSides,
    pub highlighted: MaterialsForSides,
}