pub fn get_piece_material(color: PieceColor, materials: &AppMaterials) -> MaterialTy {
    match color {
        PieceColor::WHITE => materials.pieces.default_white.clone(),
        PieceColor::BLACK => materials.pieces.default_black.clone(),
    }
}

pub fn get_piece_material_hovered(
    color: PieceColor,
    materials: &AppMaterials,
) -> MaterialTy {
    match color {
        PieceColor::WHITE => materials.pieces.highlighted_white.clone(),
        PieceColor::BLACK => materials.pieces.highlighted_black.clone(),
    }
}

pub fn get_piece_stl(ty: PieceTy) -> std::path::PathBuf {
    match ty {
        PieceTy::PAWN => "pawn.stl".into(),
        PieceTy::ROOK => "rook.stl".into(),
        PieceTy::KNIGHT => "knight.stl".into(),
        PieceTy::BISHOP => "bishop.stl".into(),
        PieceTy::QUEEN => "queen.stl".into(),
        PieceTy::KING => "king.stl".into(),
        PieceTy::MAGE => "mage.stl".into(),
    }
}

#[derive(Component)]
pub struct PieceMarker;

#[derive(Component)]
pub struct CubeMarker;

#[derive(Default)]
pub struct SelectedPieceReference(pub Option<BoardReference>);

#[derive(Component)]
pub struct CellMaterials {
    pub default: Handle<StandardMaterial>,
    pub highlighted: Handle<StandardMaterial>,
}

use bevy::{
    pbr::StandardMaterial,
    prelude::{Component, Handle},
};
use chessbik_board::{PieceColor, PieceTy};

use crate::{
    app_materials::{AppMaterials, MaterialTy},
    BoardReference,
};
