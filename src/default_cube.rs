use std::f32::consts::PI;

use crate::{
    app_materials::{AppMaterials, MaterialTy},
    *,
};

pub struct DefaultCubeQueryItem {
    pub position: Vec3,
    pub cell: Cell,
}
pub struct DefaultCubeQuery {
    pub direction: Quat,
    pub piece_mirrored: bool,
    pub cells: [DefaultCubeQueryItem; 9],
    pub default_material_getter: fn(&AppMaterials) -> MaterialTy,
    pub highlighted_material_getter: fn(&AppMaterials) -> MaterialTy,
}

lazy_static::lazy_static! {
    pub static ref TOP: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: (-1., 1.5, -1.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., 1.5,  0.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., 1.5,  1.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0., 1.5, -1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., 1.5,  0.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., 1.5,  1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1., 1.5, -1.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., 1.5,  0.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., 1.5,  1.).into(), cell: cell::WPAWN},
        ],
        default_material_getter: |mats| mats.cells.default.top.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.top.clone()
    };

    pub static ref LEFT: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: (-1.,  1., 1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0.,  1., 1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1.,  1., 1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: (-1.,  0., 1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0.,  0., 1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1.,  0., 1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: (-1., -1., 1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 0., -1., 1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 1., -1., 1.5).into(), cell: cell::WPAWN}
        ],
        default_material_getter: |mats| mats.cells.default.left.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.left.clone()
    };

    pub static ref FORWARD: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., -PI / 2.),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: (1.5,  1., -1.).into(), cell: cell::WKNIGHT},
            DefaultCubeQueryItem {position: (1.5,  1.,  0.).into(), cell: cell::WQUEEN},
            DefaultCubeQueryItem {position: (1.5,  1.,  1.).into(), cell: cell::WKNIGHT},
            DefaultCubeQueryItem {position: (1.5,  0., -1.).into(), cell: cell::WROOK},
            DefaultCubeQueryItem {position: (1.5,  0.,  0.).into(), cell: cell::WKING},
            DefaultCubeQueryItem {position: (1.5,  0.,  1.).into(), cell: cell::WROOK},
            DefaultCubeQueryItem {position: (1.5, -1., -1.).into(), cell: cell::WBISHOP},
            DefaultCubeQueryItem {position: (1.5, -1.,  0.).into(), cell: cell::WMAGE},
            DefaultCubeQueryItem {position: (1.5, -1.,  1.).into(), cell: cell::WBISHOP}
        ],
        default_material_getter: |mats| mats.cells.default.forward.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.forward.clone()
    };

    pub static ref RIGHT: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, -PI / 2., 0., 0.),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: ( 1.,  1., -1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0.,  1., -1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1.,  1., -1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 1.,  0., -1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0.,  0., -1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1.,  0., -1.5).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: ( 1., -1., -1.5).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0., -1., -1.5).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1., -1., -1.5).into(), cell: cell::BPAWN}
        ],
        default_material_getter: |mats| mats.cells.default.right.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.right.clone()
    };

    pub static ref BACK: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.),
        piece_mirrored: true,
        cells: [
            DefaultCubeQueryItem {position: (-1.5,  1.,  1.).into(), cell: cell::BKNIGHT},
            DefaultCubeQueryItem {position: (-1.5,  1.,  0.).into(), cell: cell::BQUEEN},
            DefaultCubeQueryItem {position: (-1.5,  1., -1.).into(), cell: cell::BKNIGHT},
            DefaultCubeQueryItem {position: (-1.5,  0.,  1.).into(), cell: cell::BROOK},
            DefaultCubeQueryItem {position: (-1.5,  0.,  0.).into(), cell: cell::BKING},
            DefaultCubeQueryItem {position: (-1.5,  0., -1.).into(), cell: cell::BROOK},
            DefaultCubeQueryItem {position: (-1.5, -1.,  1.).into(), cell: cell::BBISHOP},
            DefaultCubeQueryItem {position: (-1.5, -1.,  0.).into(), cell: cell::BMAGE},
            DefaultCubeQueryItem {position: (-1.5, -1., -1.).into(), cell: cell::BBISHOP}
        ],
        default_material_getter: |mats| mats.cells.default.back.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.back.clone()
    };

    pub static ref BOTTOM: DefaultCubeQuery = DefaultCubeQuery {
        direction: Quat::from_euler(EulerRot::XYZ, PI, 0., 0.),
        piece_mirrored: false,
        cells: [
            DefaultCubeQueryItem {position: ( 1., -1.5, -1.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., -1.5,  0.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 1., -1.5,  1.).into(), cell: cell::WPAWN},
            DefaultCubeQueryItem {position: ( 0., -1.5, -1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., -1.5,  0.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: ( 0., -1.5,  1.).into(), cell: cell::NONE},
            DefaultCubeQueryItem {position: (-1., -1.5, -1.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., -1.5,  0.).into(), cell: cell::BPAWN},
            DefaultCubeQueryItem {position: (-1., -1.5,  1.).into(), cell: cell::BPAWN}
        ],
        default_material_getter: |mats| mats.cells.default.bottom.clone(),
        highlighted_material_getter: |mats| mats.cells.highlighted.bottom.clone()
    };
}
