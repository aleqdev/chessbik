pub use super::MaterialTy;

pub mod materials_for_sides;
pub use materials_for_sides::*;

pub struct PlanesMaterials {
    pub default: MaterialsForSides,
    pub highlighted: MaterialsForSides,
}
