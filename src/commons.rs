use chessbik_board::{Board, PiecePosition};
use chessbik_commons::Cell;

pub fn get_piece_material(color: PieceColor, app_assets: &AppAssets) -> MaterialTy {
    match color {
        PieceColor::WHITE => app_assets.pieces.default_white.clone(),
        PieceColor::BLACK => app_assets.pieces.default_black.clone(),
    }
}

pub fn get_piece_material_hovered(color: PieceColor, app_assets: &AppAssets) -> MaterialTy {
    match color {
        PieceColor::WHITE => app_assets.pieces.highlighted_white.clone(),
        PieceColor::BLACK => app_assets.pieces.highlighted_black.clone(),
    }
}

#[derive(Component)]
pub struct PieceMarker;

#[derive(Component)]
pub struct PlaneMarker;

#[derive(Component)]
pub struct CubeMarker;

#[derive(Component)]
pub struct MageMoveIndicatorMarker(pub PiecePosition);

#[derive(Component)]
pub struct CubeDisplayerMarker;

#[derive(Component)]
pub struct CubeRotator(pub [CubeRotation; 3]);

#[derive(Default)]
pub struct AvailableMovesStorage(pub Vec<PieceMove>);

#[derive(Default)]
pub struct PlayerNameBuffer(pub String);

#[derive(Default)]
pub struct PlayerTokenBuffer(pub Option<PlayerToken>);

#[derive(Default)]
pub struct BoardReserve(pub Option<Board<Cell>>);

#[derive(Default)]
pub struct CubeRotationState {
    pub is_rotating: bool,
    pub mage: Option<PiecePosition>,
    pub selected_rotator: Option<[CubeRotation; 3]>,
    pub rotator_cycle: Option<usize>,
}

#[derive(Default)]
pub struct SelectedPieceReference(pub Option<BoardReference>);

#[derive(Component)]
pub struct CellMaterials {
    pub default: Handle<StandardMaterial>,
    pub highlighted: Handle<StandardMaterial>,
}

impl CellMaterials {
    pub fn from_side(side: Side, app_assets: &AppAssets) -> Self {
        match side {
            Side::TOP => Self {
                default: app_assets.planes.default.top.clone(),
                highlighted: app_assets.planes.highlighted.top.clone(),
            },
            Side::LEFT => Self {
                default: app_assets.planes.default.left.clone(),
                highlighted: app_assets.planes.highlighted.left.clone(),
            },
            Side::FORWARD => Self {
                default: app_assets.planes.default.forward.clone(),
                highlighted: app_assets.planes.highlighted.forward.clone(),
            },
            Side::RIGHT => Self {
                default: app_assets.planes.default.right.clone(),
                highlighted: app_assets.planes.highlighted.right.clone(),
            },
            Side::BACK => Self {
                default: app_assets.planes.default.back.clone(),
                highlighted: app_assets.planes.highlighted.back.clone(),
            },
            Side::BOTTOM => Self {
                default: app_assets.planes.default.bottom.clone(),
                highlighted: app_assets.planes.highlighted.bottom.clone(),
            },
        }
    }
}

use bevy::{
    pbr::StandardMaterial,
    prelude::{Component, Handle},
};
use chessbik_board::{CubeRotation, PieceColor, PieceMove};
use chessbik_commons::{PlayerToken, Side};

use crate::{
    app_assets::{AppAssets, MaterialTy},
    BoardReference,
};
