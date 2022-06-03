macro_rules! derive_wrapper {
    ($i:item) => {
        #[derive(
            ::derive_more::Deref,
            ::derive_more::DerefMut,
            ::derive_more::AsRef,
            ::derive_more::From,
            ::derive_more::Into,
        )]
        $i
    };
}

pub fn get_piece_material(color: crate::PieceColor, materials: &AppMaterials) -> MaterialTy {
    match color {
        crate::PieceColor::WHITE => materials.pieces.default_white.clone(),
        crate::PieceColor::BLACK => materials.pieces.default_black.clone(),
    }
}

pub fn get_piece_material_hovered(
    color: crate::PieceColor,
    materials: &AppMaterials,
) -> MaterialTy {
    match color {
        crate::PieceColor::WHITE => materials.pieces.highlighted_white.clone(),
        crate::PieceColor::BLACK => materials.pieces.highlighted_black.clone(),
    }
}

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

#[derive(Component)]
pub struct PieceMarker;

#[derive(Component)]
pub struct CubeMarker;

#[derive(Default)]
pub struct SelectedPieceReference(pub Option<FieldReference>);

#[derive(Component)]
pub struct CellMaterials {
    pub default: Handle<StandardMaterial>,
    pub highlighted: Handle<StandardMaterial>,
}

use bevy::{
    pbr::StandardMaterial,
    prelude::{Component, Handle},
};
pub(crate) use derive_wrapper;

use crate::{
    app_materials::{AppMaterials, MaterialTy},
    FieldReference,
};
